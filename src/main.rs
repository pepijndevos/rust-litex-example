#![no_std]
#![no_main]

use embedded_hal;
// use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::serial::Write;
use nb;
// use riscv;

extern crate panic_halt;
use litex_pac as pac;
use litex_hal as hal;
use riscv_rt::entry;

const REST     : f32 = 0.0;
const NOTE_B0  : f32 = 31.0;
const NOTE_C1  : f32 = 33.0;
const NOTE_CS1 : f32 = 35.0;
const NOTE_D1  : f32 = 37.0;
const NOTE_DS1 : f32 = 39.0;
const NOTE_E1  : f32 = 41.0;
const NOTE_F1  : f32 = 44.0;
const NOTE_FS1 : f32 = 46.0;
const NOTE_G1  : f32 = 49.0;
const NOTE_GS1 : f32 = 52.0;
const NOTE_A1  : f32 = 55.0;
const NOTE_AS1 : f32 = 58.0;
const NOTE_B1  : f32 = 62.0;
const NOTE_C2  : f32 = 65.0;
const NOTE_CS2 : f32 = 69.0;
const NOTE_D2  : f32 = 73.0;
const NOTE_DS2 : f32 = 78.0;
const NOTE_E2  : f32 = 82.0;
const NOTE_F2  : f32 = 87.0;
const NOTE_FS2 : f32 = 93.0;
const NOTE_G2  : f32 = 98.0;
const NOTE_GS2 : f32 = 104.0;
const NOTE_A2  : f32 = 110.0;
const NOTE_AS2 : f32 = 117.0;
const NOTE_B2  : f32 = 123.0;
const NOTE_C3  : f32 = 131.0;
const NOTE_CS3 : f32 = 139.0;
const NOTE_D3  : f32 = 147.0;
const NOTE_DS3 : f32 = 156.0;
const NOTE_E3  : f32 = 165.0;
const NOTE_F3  : f32 = 175.0;
const NOTE_FS3 : f32 = 185.0;
const NOTE_G3  : f32 = 196.0;
const NOTE_GS3 : f32 = 208.0;
const NOTE_A3  : f32 = 220.0;
const NOTE_AS3 : f32 = 233.0;
const NOTE_B3  : f32 = 247.0;
const NOTE_C4  : f32 = 262.0;
const NOTE_CS4 : f32 = 277.0;
const NOTE_D4  : f32 = 294.0;
const NOTE_DS4 : f32 = 311.0;
const NOTE_E4  : f32 = 330.0;
const NOTE_F4  : f32 = 349.0;
const NOTE_FS4 : f32 = 370.0;
const NOTE_G4  : f32 = 392.0;
const NOTE_GS4 : f32 = 415.0;
const NOTE_A4  : f32 = 440.0;
const NOTE_AS4 : f32 = 466.0;
const NOTE_B4  : f32 = 494.0;
const NOTE_C5  : f32 = 523.0;
const NOTE_CS5 : f32 = 554.0;
const NOTE_D5  : f32 = 587.0;
const NOTE_DS5 : f32 = 622.0;
const NOTE_E5  : f32 = 659.0;
const NOTE_F5  : f32 = 698.0;
const NOTE_FS5 : f32 = 740.0;
const NOTE_G5  : f32 = 784.0;
const NOTE_GS5 : f32 = 831.0;
const NOTE_A5  : f32 = 880.0;
const NOTE_AS5 : f32 = 932.0;
const NOTE_B5  : f32 = 988.0;
const NOTE_C6  : f32 = 1047.0;
const NOTE_CS6 : f32 = 1109.0;
const NOTE_D6  : f32 = 1175.0;
const NOTE_DS6 : f32 = 1245.0;
const NOTE_E6  : f32 = 1319.0;
const NOTE_F6  : f32 = 1397.0;
const NOTE_FS6 : f32 = 1480.0;
const NOTE_G6  : f32 = 1568.0;
const NOTE_GS6 : f32 = 1661.0;
const NOTE_A6  : f32 = 1760.0;
const NOTE_AS6 : f32 = 1865.0;
const NOTE_B6  : f32 = 1976.0;
const NOTE_C7  : f32 = 2093.0;
const NOTE_CS7 : f32 = 2217.0;
const NOTE_D7  : f32 = 2349.0;
const NOTE_DS7 : f32 = 2489.0;
const NOTE_E7  : f32 = 2637.0;
const NOTE_F7  : f32 = 2794.0;
const NOTE_FS7 : f32 = 2960.0;
const NOTE_G7  : f32 = 3136.0;
const NOTE_GS7 : f32 = 3322.0;
const NOTE_A7  : f32 = 3520.0;
const NOTE_AS7 : f32 = 3729.0;
const NOTE_B7  : f32 = 3951.0;
const NOTE_C8  : f32 = 4186.0;
const NOTE_CS8 : f32 = 4435.0;
const NOTE_D8  : f32 = 4699.0;
const NOTE_DS9 : f32 = 4978.0;

const MELODY: [f32; 216] = [
  //A
  NOTE_FS5, NOTE_GS5, NOTE_DS5, NOTE_DS5, REST, NOTE_B4, NOTE_D5, NOTE_CS5, NOTE_B4, REST, NOTE_B4, NOTE_CS5, NOTE_D5, NOTE_D5, NOTE_CS5, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_FS5, NOTE_GS5, NOTE_DS5, NOTE_FS5, NOTE_CS5,
  NOTE_DS5, NOTE_B4, NOTE_CS5, NOTE_B4, NOTE_DS5, NOTE_FS5, NOTE_GS5, NOTE_DS5, NOTE_FS5, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_D5, NOTE_DS5, NOTE_D5, NOTE_CS5, NOTE_B4, NOTE_CS5, NOTE_D5, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_FS5,
  NOTE_CS5, NOTE_DS5, NOTE_CS5, NOTE_B4, NOTE_CS5, NOTE_B4, NOTE_CS5, NOTE_FS5, NOTE_GS5, NOTE_DS5, NOTE_DS5, REST, NOTE_B4, NOTE_D5, NOTE_CS5, NOTE_B4, REST, NOTE_B4, NOTE_CS5, NOTE_D5, NOTE_D5, NOTE_CS5, NOTE_B4,
  NOTE_CS5, NOTE_DS5, NOTE_FS5, NOTE_GS5, NOTE_DS5, NOTE_FS5, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_CS5, NOTE_B4, NOTE_DS5, NOTE_FS5, NOTE_GS5, NOTE_DS5, NOTE_FS5, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_D5, NOTE_DS5,
  NOTE_D5, NOTE_CS5, NOTE_B4, NOTE_CS5, NOTE_D5, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_FS5, NOTE_CS5, NOTE_DS5, NOTE_CS5, NOTE_B4, NOTE_CS5, NOTE_B4, NOTE_CS5, //end of loop
  NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_E5, NOTE_DS5, NOTE_E5, NOTE_FS5, NOTE_B4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_FS4, NOTE_E5,
  NOTE_DS5, NOTE_CS5, NOTE_B4, NOTE_FS4, NOTE_DS4, NOTE_E4, NOTE_FS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_FS4,
  //
  NOTE_B4, NOTE_B4, NOTE_AS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_E4, NOTE_E5, NOTE_DS5, NOTE_E5, NOTE_FS5, NOTE_B4, NOTE_AS4,
  //
  NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_E5, NOTE_DS5, NOTE_E5, NOTE_FS5, NOTE_B4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_FS4, NOTE_E5, NOTE_DS5,
  NOTE_CS5, NOTE_B4, NOTE_FS4, NOTE_DS4, NOTE_E4, NOTE_FS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_B4, NOTE_CS5, NOTE_DS5, NOTE_B4, NOTE_FS4,
  NOTE_GS4, NOTE_FS4, NOTE_B4, NOTE_B4, NOTE_AS4, NOTE_B4, NOTE_FS4, NOTE_GS4, NOTE_B4, NOTE_E5, NOTE_DS5, NOTE_E5, NOTE_FS5, NOTE_B4, NOTE_CS5 
];

const DURATION: [u32; 216] = [
  8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 16, 16, 16, 16, 16, 16, 16, 16, 8,
  8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 16, 16, 16, 16, 16, 16,
  16, 16, 8, 8, 8, //
  8, 16, 16, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 16, 16, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, //
  8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, //
  8, 16, 16, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 16, 16, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, //
  8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 8, 8 
];


hal::uart! {
    UART: pac::UART,
}

// hal::gpio! {
//     LEDS: pac::LEDS,
// }

hal::timer! {
    TIMER: pac::TIMER0,
}

fn f2p(f:f32) -> u32 {
    return (60_000_000.0/f) as u32
}

fn play(delay: &mut TIMER, stp: &pac::STEPPER1, f:f32, dur:f32) {
    let p = f2p(f);
    stp.period.write(|w| unsafe { w.bits(p) });
    stp.steps.write(|w| unsafe { w.bits((f*dur) as u32) });
    delay.delay_ms((dur*1000.0) as u32);
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
    // LEDS { index: 0 }.set_high().unwrap();

    let mut delay = TIMER {
        registers: peripherals.TIMER0,
        sys_clk: 60_000_000,
    };

    serial.bwrite_all(b"turn\n").unwrap();
    for n in 0..216 {
        let note = MELODY[n]/2.0;
        let dur = 2.0/DURATION[n] as f32;
        play(&mut delay, &peripherals.STEPPER1, note, dur);
        delay.delay_ms(10 as u32);
    }
    peripherals.CTRL.reset.write(|w| unsafe { w.bits(1) });

    loop { }
}
