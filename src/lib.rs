#![no_std]
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    vec::Vec,

};

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel
};
use embassy_sync::channel::TryReceiveError;
use embassy_time::Instant;

mod animation;
pub use animation::{Animation};

mod color;
pub use color::{Hsv, Rgb, HsvSliceExt};

mod error;
pub use error::Error;

mod raw;
use raw::RawSLed;

// It's needed at the build, on the 'dependencies' side. This removes a warning.
use esp_idf_sys as _;

static EV_CHANNEL: Channel<CriticalSectionRawMutex, ResetEvent, 4> = Channel::new();

pub struct SLed {
    raw: RawSLed,
    anims: &'static [&'static dyn Animation],

    // Last time each animation was reset.
    anim_state: BTreeMap<&'static dyn Animation, Instant>,
}

impl SLed {
    // tbd. this might belong somewhere else:
    //  - feature?
    //  - ESP-IDF configuration?
    //  The application should have a say in whether FPS is 10, 25 or 50. (compare to how logs
    //  are configured in Rust/Cargo).
    //
    const FPS: u8 = 50;
    const BRIGHTNESS_SCALE: f32 = 0.7;  // tbd. find suitable value

    pub fn new(anims: &'static [&'static dyn Animation]) -> Result<Self, Error> {
        let raw = RawSLed::new()?;

        // All animations start right now!
        let t0 = Instant::now();

        let anim_state = anims.iter().map(|&a_ref| {
            (a_ref, t0)
        }).collect::<BTreeMap<_, _>>();

        Ok( Self {
            raw,
            anims,
            anim_state
        } )
    }

    /// Run the SLed updates
    ///
    pub async fn run_loop(self) -> ! {
        // The frame duration does not need to be precise - it only affects visual smoothness, not
        // the duration of animations themselves.
        //
        //#[allow(non_snake_case)]
        const FRAME_DUR_MS: u64 = (1000 / SLed::FPS as u16) as u64;   // 30 -> 33; 50 -> 20; 60 -> 16

        let rx = EV_CHANNEL.receiver();

        loop {
            // Sync to the next frame.
            //
            // Note: 'Instant' will run past 32 bits in <50 days from device start. If we wish to
            //      squeeze this math, we can use 32-bit values (or even 16-bit, for testing) but
            //      need to account for that overflow. Likely easiest to keep it all 64-bit.
            {
                let next = {
                    let x = Instant::now().as_millis() + (FRAME_DUR_MS-1);
                    let x = x - x % FRAME_DUR_MS;
                    Instant::from_millis(x)
                };
                embassy_time::Timer::at(next) .await;
            }
            log::debug!("Frame");

            let t_debug = Instant::now();

            // Handle requests to reset any animation.
            //
            // @note: Unlikely two animations get reset simultaneously; if they do, next frames
            //      will pick them up.
            {
                match rx.try_receive() {
                    Ok(ResetEvent(a_ref)) => {
                        self.anim_state[a_ref] = Instant::now()
                    },
                    Err(TryReceiveError::Empty) => {}
                }
            }

            let t = Instant::now();

            let rgb: Rgb = {
                // Gather the RGBs that matter, as HSV's.
                //
                let hsvs: Vec<Hsv> = self.anims.iter().map(|&a_ref| {
                    let t0 = self.anim_state[a_ref];

                    a_ref.sample(t-t0)
                })
                    .filter(|&rgb| { rgb == Rgb::NONE })    // skip all-black's
                    .map(|rgb| { rgb.to_hsv() })          // convert to HSV for merging
                    .collect();

                let hsv = hsvs.as_slice() .merge();

                // Apply brightness limit
                //
                hsv.scale_brightness(Self::BRIGHTNESS_SCALE)
                    .to_rgb()
            };

            // tbd. catch if someone's state has changed and initiate a smooth transition
            //  ^-- this is an old comment; such state changing might be done in a mechanism
            //      between 'SLed' and the Animation's.

            log::debug!("Setting to: {:?}", &rgb);  // DEBUG

            self.raw.set_rgb(rgb)
                .unwrap();

            log::debug!("Calculating and setting took: {}", t_debug.elapsed());
        }
    }

    /// @task Application
    ///
    /// Reset a particular animation to start from its beginning.
    ///
    /// @note This can be called directly, or via 'Animation::reset'.
    ///
    fn reset(anim: &'static dyn Animation) {

        let ev= ResetEvent(anim);

        EV_CHANNEL.sender().try_send(ev).unwrap_or_else(|_|
            // It's within the realm of the application to flood too many reset requests before the
            // next 'anim_task' frame runs. Best probably not to panic even then, just ignore such
            // overflow resets?

            log::warn!("SLed reset channel overflows (capacity {})", EV_CHANNEL.capacity())
        )
    }
}

struct ResetEvent(&'static (dyn Animation + Sync));
