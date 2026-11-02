#![no_std]
extern crate alloc;

mod raw;

use alloc::collections::BTreeMap;
use embassy_time::{Duration, Instant};
use raw::RawSLed;

mod color;
pub use color::{Hsv, Rgb};

pub mod animation;   // 'animator::pulse' etc.
pub use animation::Animation;

mod error;
pub use error::Error;

// It's needed at the build, on the 'dependencies' side. This removes a warning.
use esp_idf_sys as _;

pub struct SLed {
    raw: RawSLed,
    anims: &'static [&'static dyn Animation],

    // For each animation, where are they currently at.
    anim_state: BTreeMap<&'static dyn Animation, AnimState>
}

impl SLed {
    // tbd. this might belong somewhere else:
    //  - feature?
    //  - const generic?
    //  The application should have a say in whether FPS is 10, 25 or 50.
    const FPS: u8 = 50;

    pub fn new(anims: &'static [&'static dyn Animation]) -> Result<Self, Error> {
        let raw = RawSLed::new()?;

        let anim_state = anims.map(|a| {
            AnimState::new(a, &raw)
        });

        Ok( Self {
            raw,
            anims,
            anim_state
        } )
    }

    /// Run the SLed updates
    ///
    pub async fn run_loop(self) -> ! {
        // The frame duration does not need to be precise - it only affects the refresh rate (visuals).
        //#[allow(non_snake_case)]
        const FRAME_DUR_MS: u64 = (1000 / SLed::FPS as u16) as u64;   // 30 -> 33; 50 -> 20; 60 -> 16

        //r let t0 = Instant::now();

        loop {
            // Sync to the next frame by modulo.
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

            // On the 'anims', we should visit them

            let rgb = {
                todo!()

                // #hack We just take the initial animator, for now.
                //let a = self.anims[0];

                //a.sample(t)
            };

            // tbd. merge the results; do multiplexer as Animator
            // tbd. catch if someone's state has changed as initiate a smooth transition

            log::debug!("Setting to: {:?}", &rgb);  // DEBUG

            self.raw.set_rgb(rgb)
                .unwrap();

            log::debug!("Calculating and setting took: {}", t_debug.elapsed());
        }
    }
}

/// tbd. comment
struct AnimState {
    /// The animation that provides (f: 0.0 .. 1.0) -> Option<Rgb> (but nothing else!)
    a: &'static dyn Animation,

    t0: Instant,
    dur: Duration,
}

impl AnimState {

    fn restart(&mut self) {

    }
}
