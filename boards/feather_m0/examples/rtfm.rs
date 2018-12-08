#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::entry;
use rtfm::app;

app! {
    device: hal,
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn init(_p: init::Peripherals) -> () {
}

#[entry]
fn run_app() -> ! {
    main();
    loop {}
}
