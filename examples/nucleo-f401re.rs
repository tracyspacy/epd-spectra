//! Simple no-std "Hello World" example for the STM32 Nucleo F401RE microcontroller board
//! with a 2.66 inch e-paper display. Similiar to the eink example of the
//! [nucleo-f401re crate](https://github.com/jkristell/nucleo-f401re/) which uses the
//! [epd-waveshare crate](https://github.com/caemor/epd-waveshare/) instead.
//!
//! Connections:
//!
//! | Nucleo | EPD   |
//! |--------|-------|
//! | PB3    | SCK   |
//! | PB5    | MOSI  |
//! | PA6    | CS    |
//! | PA7    | BUSY  |
//! | PB6    | DC    |
//! | PA9    | RESET |
//!
//! If you have another display size, simply replace `Display2in66` with your display.
//! To run this example clone this repository and run:
//! `cargo run --example nucleo-f401re --target thumbv7em-none-eabihf`
//! see also: <https://github.com/jkristell/nucleo-f401re>

#![no_main]
#![no_std]
#![cfg(target_os = "none")]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use nucleo_f401re::{
    hal::{
        prelude::*,
        spi::{self, Spi},
    },
    pac, Led,
};

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{iso_8859_1::FONT_10X20, MonoTextStyle},
    prelude::*,
    text::Text,
};
use epd_spectra::{Display2in66, Epd, TriColor};

#[allow(clippy::similar_names)]
#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();

    // (Re-)configure PA5 (LD2 - User Led) as output
    let mut led = Led::new(gpioa.pa5);
    led.set(false);

    // Constrain clock registers
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    // Configure GPIO pins
    let busy = gpioa.pa7.into_floating_input();
    let dc = gpiob.pb6.into_push_pull_output();
    let reset = gpioa.pa9.into_push_pull_output();

    // Configure SPI
    let sck = gpiob.pb3.into_alternate();
    let miso = spi::NoMiso::new();
    let mosi = gpiob.pb5.into_alternate();
    let cs = gpioa.pa6.into_push_pull_output();
    let spi = Spi::new(
        device.SPI1,
        (sck, miso, mosi),
        epd_spectra::SPI_MODE,
        4.MHz(),
        &clocks,
    );
    let mut spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs);

    // create EPD driver
    let epd = Epd::new(&mut spi_device, busy, dc, reset, &mut delay, 0);
    let mut epd = epd.init(&mut spi_device, &mut delay).unwrap();

    let mut display = Display2in66::default();
    Text::new(
        "Hello",
        Point::new(10, 20),
        MonoTextStyle::new(&FONT_10X20, TriColor::Black),
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "World",
        Point::new(30, 60),
        MonoTextStyle::new(&FONT_10X20, TriColor::Red),
    )
    .draw(&mut display)
    .unwrap();

    let ferris: ImageRaw<TriColor> = ImageRaw::new(FERRIS_IMG, FERRIS_WIDTH);
    let ferris: Image<_> = Image::new(&ferris, Point::new(0, 100));
    ferris.draw(&mut display).unwrap();

    epd.update(&display, &mut spi_device, &mut delay).unwrap();
    let _inactive_epd = epd.power_off(&mut spi_device, &mut delay).unwrap();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}

// This code is generated with convert_bmp.py
const FERRIS_WIDTH: u32 = 150;
const FERRIS_IMG: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 10, 160, 0, 42, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 168, 0, 42, 160, 0,
    170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 170, 0, 42, 168, 2, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 2, 170, 128, 170, 168, 10, 170, 128, 0, 160, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 128, 10, 170, 162, 170, 170,
    42, 170, 128, 10, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 42, 160, 10, 170, 170, 170, 170, 170, 170, 160, 42, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 170, 42, 170, 170, 170, 170, 170, 170, 160,
    170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    168, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 168, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 160, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 42, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 0, 0, 0, 0, 160,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 0, 0, 0, 0, 0, 0, 2, 160, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0,
    0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0,
    0, 0, 0, 42, 160, 0, 32, 0, 0, 0, 0, 160, 0, 0, 0, 0, 2, 170, 130, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 10, 170, 0, 0, 0, 0, 170, 160, 0, 40, 0,
    0, 0, 0, 170, 0, 0, 0, 0, 2, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 2, 170, 128, 0, 40, 0, 0, 0, 0, 170, 128, 0, 0, 0,
    0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 0, 0, 0, 10, 170, 128, 0, 40, 0, 0, 40, 0, 170, 160, 0, 0, 0, 0, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 42,
    170, 128, 0, 168, 0, 0, 168, 0, 170, 168, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 168, 0, 0, 0, 170, 170, 128, 0, 170, 0,
    2, 168, 0, 42, 170, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 168, 0, 0, 0, 170, 170, 128, 2, 170, 0, 2, 170, 0, 42, 170,
    128, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 168, 0, 0, 2, 170, 170, 128, 2, 170, 0, 10, 170, 0, 42, 170, 128, 0, 0, 0, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 168,
    0, 0, 2, 170, 170, 128, 10, 170, 0, 10, 170, 128, 42, 170, 160, 0, 42, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 160, 2,
    170, 170, 128, 42, 168, 0, 10, 170, 128, 42, 170, 160, 0, 42, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 168, 2, 170,
    170, 128, 170, 168, 0, 10, 170, 160, 42, 170, 160, 0, 42, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 168, 2, 170, 170,
    138, 170, 168, 0, 10, 170, 168, 42, 170, 160, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 160, 2, 170, 170, 170,
    170, 160, 0, 10, 170, 168, 10, 170, 160, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 160, 2, 170, 170, 170, 170,
    160, 0, 10, 170, 170, 10, 170, 160, 0, 10, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 128, 0, 170, 170, 170, 170, 128, 0,
    2, 170, 170, 170, 170, 160, 0, 10, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 128, 0, 170, 170, 170, 170, 0, 0, 2, 170,
    170, 170, 170, 160, 0, 2, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 128, 0, 42, 170, 170, 170, 0, 0, 0, 170, 170, 170, 170,
    128, 0, 2, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 128, 0, 42, 170, 170, 168, 0, 0, 0, 42, 170, 170, 170, 128, 0, 42,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 168, 0, 10, 170, 170, 160, 0, 0, 0, 10, 170, 170, 170, 128, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 168, 2, 170, 170, 128, 0, 0, 0, 0, 170, 170, 170, 2, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 2, 170, 168, 0, 0, 0, 0, 0, 42, 170, 168, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 2, 170,
    160, 0, 0, 0, 0, 0, 2, 170, 168, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 1, 86,
    170, 170, 170, 160, 21, 170, 170, 170, 170, 170, 170, 170, 170, 168, 10, 170, 0, 0, 0, 0, 0, 0,
    0, 42, 168, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170, 168, 0, 85, 170, 170, 170, 128, 5,
    106, 170, 170, 170, 170, 170, 170, 170, 160, 42, 168, 0, 0, 0, 0, 0, 0, 0, 42, 170, 0, 10, 170,
    170, 170, 170, 170, 170, 170, 170, 160, 0, 85, 106, 170, 170, 0, 5, 90, 170, 170, 170, 170,
    170, 170, 170, 160, 170, 160, 0, 0, 0, 0, 0, 0, 0, 10, 170, 128, 10, 170, 170, 170, 170, 170,
    170, 170, 170, 160, 0, 85, 106, 170, 170, 0, 5, 90, 170, 170, 170, 170, 170, 170, 170, 130,
    170, 160, 0, 0, 0, 0, 0, 0, 0, 2, 170, 168, 2, 170, 170, 170, 170, 170, 170, 170, 170, 160, 0,
    85, 90, 170, 169, 0, 5, 90, 170, 170, 170, 170, 170, 170, 170, 138, 170, 128, 0, 0, 0, 0, 0, 0,
    0, 0, 170, 170, 138, 170, 170, 170, 170, 170, 170, 170, 170, 164, 0, 85, 90, 170, 169, 64, 5,
    86, 170, 170, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 164, 1, 85, 90, 170, 169, 80, 21, 86, 170, 170, 170, 170,
    170, 170, 170, 170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 165, 69, 85, 90, 170, 169, 85, 85, 86, 170, 170, 170, 170, 170, 170, 170, 170, 160,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 165, 85, 85,
    90, 170, 169, 85, 85, 86, 170, 170, 170, 170, 170, 170, 170, 170, 128, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 10, 170, 170, 170, 170, 170, 170, 170, 170, 170, 165, 85, 85, 90, 170, 169, 85, 85, 86,
    170, 170, 170, 170, 170, 170, 170, 170, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 165, 85, 85, 106, 170, 170, 85, 85, 90, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 169, 85, 85, 106, 170, 170, 149, 85, 106, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 85, 85,
    170, 170, 170, 165, 85, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 160, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 165, 90, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 10, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 42, 170, 168, 42, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 10, 130, 170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 42, 170, 160, 42, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 160, 42, 130,
    170, 160, 0, 0, 0, 0, 0, 0, 0, 0, 10, 170, 168, 10, 160, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 149, 86, 170, 170, 170, 170, 170, 170, 170, 170, 0, 42, 2, 170, 160, 0, 0, 0, 0,
    0, 0, 0, 0, 2, 170, 168, 2, 160, 2, 170, 170, 170, 170, 170, 170, 170, 170, 169, 85, 86, 170,
    170, 170, 170, 170, 170, 170, 160, 0, 40, 2, 170, 128, 0, 0, 0, 0, 0, 0, 0, 0, 2, 170, 168, 0,
    168, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170, 85, 90, 170, 170, 170, 170, 170, 170, 168,
    0, 0, 168, 10, 170, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 0, 168, 0, 0, 42, 170, 170, 170,
    170, 170, 170, 170, 165, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 160, 10, 170, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 42, 170, 128, 42, 0, 0, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170, 170,
    170, 170, 170, 170, 128, 0, 0, 2, 128, 10, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 170, 128, 10,
    0, 0, 0, 0, 42, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 0, 0, 0, 0, 2, 128, 42,
    168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 170, 160, 2, 128, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170,
    170, 170, 170, 168, 0, 0, 0, 0, 0, 10, 0, 42, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 160,
    0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 168, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 42, 128, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 42, 168, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 32, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 168, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 170, 0, 8, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 170, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 160, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 10, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 168, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 128, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
