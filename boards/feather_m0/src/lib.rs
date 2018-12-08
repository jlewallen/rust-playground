#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd21_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;

pub use hal::atsamd21g18a::*;
pub use hal::*;

use hal::prelude::*;
use hal::atsamd21g18a::gclk::clkctrl::GENR;
use hal::atsamd21g18a::gclk::genctrl::SRCR;

use hal::gpio::{Floating, Input, Port};
use hal::gpio::IntoFunction;
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4};
use hal::time::Hertz;

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

define_pins!(
    struct Pins,
    target_device: atsamd21g18a,

    pin a0 = a2,
    pin a1 = b8,
    pin a2 = b9,
    pin a3 = a4,
    pin a4 = a5,
    pin a5 = b2,

    pin tx = b22,
    pin rx = b23,

    pin d0 = a11,
    pin d1 = a10,
    pin d5 = a15,
    pin d6 = a20,
    pin d9 = a7,
    pin d10 = a18,
    pin d11 = a16,
    pin d12 = a19,
    pin d13 = a17,

    pin sda = a22,
    pin scl = a23,

    pin sck = b11,
    pin mosi = b10,
    pin miso = a12,

    pin usb_dm = a24,
    pin usb_dp = a25,
);

pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: SERCOM4,
    pm: &mut PM,
    sck: gpio::Pb11<Input<Floating>>,
    mosi: gpio::Pb10<Input<Floating>>,
    miso: gpio::Pa12<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster4 {
    let gclk0 = clocks.gclk0();
    SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        hal::sercom::SPI4Pinout::Dipo0Dopo1 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    )
}

pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: SERCOM3,
    pm: &mut PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster3 {
    let gclk0 = clocks.gclk0();
    I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom3,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

#[cfg(feature = "usb")]
pub fn usb_bus(
    usb: USB,
    clocks: &mut GenericClockController,
    pm: &mut PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusWrapper<UsbBus> {
    let clk6 = clocks.configure_gclk_divider_and_source(GENR::GCLK6, 1, SRCR::DFLL48M, false).unwrap();
    let usb_clock = &clocks.usb(&clk6).unwrap();
    UsbBusWrapper::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
