#![feature(adt_const_params)]
#![feature(min_adt_const_params)]
#![feature(const_param_ty_trait)]
#![no_std]
extern crate alloc;

use core::marker::ConstParamTy;

use alloc::{
    collections::BTreeMap,
    vec::Vec,
};

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel
};
use embassy_sync::channel::TryReceiveError;
use embassy_time::{Duration, Instant, Ticker};

mod animation;
pub use animation::{Animation, AnimationKey};

#[cfg(feature = "anims")]
pub mod anims;

mod color;
pub use color::{Hsv, Rgb};

mod error;
pub use error::Error;

mod raw;
use raw::RawSLed;

// It's needed at the build, on the 'dependencies' side. This removes a warning.
use esp_idf_sys as _;

static EV_CHANNEL: Channel<CriticalSectionRawMutex, ResetEvent, 4> = Channel::new();

/// Project specific configuration for the LED output.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub struct SLedConfig {
    /// How many times per second the LED is updated
    fps: u8,
    /// Ceiling (scaled, not clamped) for the brightness of the output. Viewer safety.
    brightness_limit: u8    // 1..100
}

impl SLedConfig {
    /// Provide a default that is also accessible for application code (for basing custom configs).
    ///
    /// @note _not_ using 'impl Default' because we need to be 'const fn'.
    ///
    pub const fn default() -> Self {
        Self {
            fps: 30,
            brightness_limit: 70
        }
    }
}

// Note: 'CFG' const generic with a default needs to be given here, not for the '.run_loop()' only
//      (though it's the only place needing it). Const generic for a function is not allowed a default.
//
pub struct SLed<const CFG: SLedConfig /*r = { SLedConfig::default() }*/> {
    raw: RawSLed,
    anims: &'static [&'static dyn Animation],

    // Last time each animation was reset.
    anim_state: BTreeMap<AnimationKey, Instant>,
}

impl<const CFG: SLedConfig> SLed<CFG> {

    pub fn new_with_config(anims: &'static [&'static dyn Animation]) -> Result<Self, Error> {
        let raw = RawSLed::new()?;

        // All animations start right now!
        let t0 = Instant::now();

        let anim_state = anims.iter().map(|&a_ref| {
            (AnimationKey(a_ref), t0)
        }).collect::<BTreeMap<_, _>>();

        Ok( Self {
            raw,
            anims,
            anim_state
        } )
    }

    /// Run the SLed updates
    ///
    pub async fn run_loop(mut self) -> ! {
        // Note: This only affects _smoothness_ of visual animation, not the timing of the animations
        //      themselves.
        //
        let mut ticker = {
            #[allow(non_snake_case)]
            let FRAME_MS: u64 = (1000.0 / CFG.fps as f32).round() as u64;   // 30 -> 33; 50 -> 20; 60 -> 17
            Ticker::every(Duration::from_millis(FRAME_MS))
        };

        #[allow(non_snake_case)]
        let BRIGHTNESS_LIMIT: f32 = CFG.brightness_limit as f32 * 0.01;

        let rx = EV_CHANNEL.receiver();

        log::debug!("Entering frame loop..");

        let mut _count = 0;

        loop {
            if _count % 30 == 0 {
                log::debug!("Frame #{}", _count);
            }
            _count += 1;

            //r let t_debug = Instant::now();

            // Handle requests to reset any animation.
            //
            // @note: Unlikely two animations get reset simultaneously; if they do, next frames
            //      will pick them up.
            {
                match rx.try_receive() {
                    Ok(ResetEvent(a_key)) => {
                        log::debug!("Reset event");
                        //self.anim_state[&a_key] = Instant::now()

                        assert!(self.anim_state.contains_key(&a_key));
                        self.anim_state.insert(a_key, Instant::now());
                    },
                    Err(TryReceiveError::Empty) => {}
                }
            }

            let t = Instant::now();

            let rgb: Rgb = {
                // Gather the RGBs that matter.
                //
                let rgbs: Vec<Rgb> = self.anims.iter()
                    .map(|&a| AnimationKey(a))
                    .filter_map(|a_key| {
                        let t0 = self.anim_state[&a_key];

                        let x = a_key.sample(t-t0);
                        log::debug!("Collected: {:?}", x);

                        if x == Rgb::NONE {
                            None    // all-black: treat as transparent (no effect, at all)
                        } else {
                            Some(x)
                        }
                    })
                    //
                    .collect();

                log::debug!("Collected: {}", rgbs.len());

                if rgbs.is_empty() {
                    Rgb::NONE
                } else {
                    // Note: Though much use could be a single color only, there's no point in handling
                    //      that separately. For one, brightness limiting happens in HSV so we'd need
                    //      the conversion, anyhow.
                    //
                    let hsvs = rgbs.into_iter().map(|rgb| { rgb.to_hsv() });
                    log::debug!("hsvs: {:?}", hsvs);
                    let hsv = Hsv::merge_from(hsvs);

                    log::debug!("Before scaling: {:?}", hsv);

                    // Apply brightness limit
                    //
                    let hsv = hsv.scale_brightness(BRIGHTNESS_LIMIT);

                    log::debug!("After scaling: {:?}", hsv);

                    hsv.to_rgb()
                }
            };

            log::debug!("Setting to: {:?}", rgb);  // DEBUG

            self.raw.set_rgb(rgb)
                .unwrap();

            //log::debug!("Calculating and setting took: {}ms", t_debug.elapsed().as_millis());
                // measured:
                //   - 'ButtonPing' that's giving 'NONE' (baseline):   0..1ms (~700..1690 ticks)
                //

            // Next frame
            ticker.next().await;
        }
    }
}

impl SLed<{ SLedConfig::default() }> {
    pub fn new(anims: &'static [&'static dyn Animation]) -> Result<Self, Error> {
        Self::new_with_config(anims)
    }
}

/// @task Application
///
/// Reset a particular animation to start from its beginning.
///
/// @note Called via 'Animation::restart'.
///
fn reset_anim(a_key: AnimationKey) {
    let ev= ResetEvent(a_key);

    EV_CHANNEL.sender().try_send(ev).unwrap_or_else(|_|
        // It's within the realm of the application to flood too many reset requests before the
        // next 'anim_task' frame runs. Best probably not to panic even then, just ignore such
        // overflow resets?

        log::warn!("SLed reset channel overflows (capacity {})", EV_CHANNEL.capacity())
    )
}

struct ResetEvent(AnimationKey);
