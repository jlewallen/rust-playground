#![no_std]
#![no_main]

extern crate cortex_m;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

use cortex_m::asm;

pub use hal::atsamd21g18a::{Peripherals, CorePeripherals};
pub use hal::atsamd21g18a::interrupt;

use hal::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom5Pad2, Sercom5Pad3, UART5Pinout, UART5};
use hal::target_device::gclk::clkctrl::GENR;
use hal::target_device::gclk::genctrl::SRCR;
use hal::delay::Delay;
use core::fmt::Write;

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;

static LED: Mutex<RefCell<Option<hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>>>> = Mutex::new(RefCell::new(None));
static DEBUG: Mutex<RefCell<Option<UART5>>> = Mutex::new(RefCell::new(None));
static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(mut peripherals) = Peripherals::take() {
        let mut core = CorePeripherals::take().unwrap();
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = hal::Pins::new(peripherals.PORT);

        clocks.configure_gclk_divider_and_source(GENR::GCLK2, 1, SRCR::DFLL48M, false);
        let gclk2 = clocks.get_gclk(GENR::GCLK2).unwrap();
        let rx_pin: Sercom5Pad3 = pins
            .rx
            .into_pull_down_input(&mut pins.port)
            .into_pad(&mut pins.port);
        let tx_pin: Sercom5Pad2 = pins
            .tx
            .into_push_pull_output(&mut pins.port)
            .into_pad(&mut pins.port);
        let uart_clk = clocks
            .sercom5_core(&gclk2)
            .unwrap();

        let uart = UART5::new(
            &uart_clk,
            115200.hz(),
            peripherals.SERCOM5,
            &mut core.NVIC,
            &mut peripherals.PM,
            UART5Pinout::Rx3Tx2 {
                rx: rx_pin,
                tx: tx_pin,
            },
        );

        let delay = Delay::new(core.SYST, &mut clocks);
        let led = pins.d13.into_open_drain_output(&mut pins.port);

        cortex_m::interrupt::free(move |cs| {
            *DEBUG.borrow(cs).borrow_mut() = Some(uart);
            *DELAY.borrow(cs).borrow_mut() = Some(delay);
            *LED.borrow(cs).borrow_mut() = Some(led);
        });
    }

    loop {
        cortex_m::interrupt::free( |cs| {
            if let (&mut Some(ref mut dbg), &mut Some(ref mut led)) = (DEBUG.borrow(cs).borrow_mut().deref_mut(), LED.borrow(cs).borrow_mut().deref_mut()) {
                dbg.write_str("Hello, world!\n").unwrap();
                led.set_low();
            }
        });
    }
}

interrupt!(SERCOM5, sercom5_handler);
fn sercom5_handler() {
    cortex_m::interrupt::free( |cs| {
        if let (&mut Some(ref mut dbg), &mut Some(ref mut led)) = (DEBUG.borrow(cs).borrow_mut().deref_mut(), LED.borrow(cs).borrow_mut().deref_mut()) {
            dbg.read().unwrap();
            led.set_high();
        }
    });
}
