#![no_std]
#![no_main]

use embedded_hal;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::serial::Write;
use nb;
use riscv;

extern crate panic_halt;
use litex_pac as pac;
use litex_hal as hal;
use riscv_rt::entry;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyleBuilder,
    style::TextStyleBuilder,
};
use ssd1306::{prelude::*, Builder};

hal::uart! {
    UART: pac::UART,
}

hal::gpio! {
    CTL: pac::OLED_CTL,
    LEDS: pac::LEDS,
}

hal::spi! {
    SPI: (pac::OLED_SPI, u8),
}

hal::timer! {
    TIMER: pac::TIMER0,
}

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut serial = UART {
        registers: peripherals.UART,
    };

    serial.bwrite_all(b"Hello world!\n").unwrap();

    for i in 0..8 {
        if i % 2 == 0 {
            LEDS { index: i }.set_high().unwrap();
        }
    }

    let mut dc = CTL { index: 0 };
    let mut rstn = CTL { index: 1 };
    let mut csn = CTL { index: 2 };
    let mut spi = SPI {
        registers: peripherals.OLED_SPI
    };
    let mut delay = TIMER {
        registers: peripherals.TIMER0,
        sys_clk: 50_000_000,
    };

    csn.set_low().unwrap();
    let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
    let mut disp: GraphicsMode<_> = Builder::new().with_rotation(DisplayRotation::Rotate180).connect(interface).into();

    disp.reset(&mut rstn, &mut delay).unwrap();
    disp.init().unwrap();
    loop {
        disp.clear();

        let yoffset = 20;

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();

        // screen outline
        // default display size is 128x64 if you don't pass a _DisplaySize_
        // enum to the _Builder_ struct
        Rectangle::new(Point::new(0, 0), Point::new(127, 63))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        // triangle
        Triangle::new(
            Point::new(16, 16 + yoffset),
            Point::new(16 + 16, 16 + yoffset),
            Point::new(16 + 8, yoffset),
        )
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

        // square
        Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        // circle
        Circle::new(Point::new(96, yoffset + 8), 8)
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        disp.flush().unwrap();

        delay.delay_ms(1000 as u32);
        disp.clear();
        // text

        let text_style = TextStyleBuilder::new(Font6x8)
            .text_color(BinaryColor::On)
            .build();

        Text::new("Hello world!", Point::zero())
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        Text::new("Hello Rust!", Point::new(0, 16))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        disp.flush().unwrap();
        delay.delay_ms(1000 as u32);

        for i in 0..8 {
            LEDS { index: i }.toggle().unwrap();
        }
    }
}
