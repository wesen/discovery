#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate f3;
extern crate pg;

use pg::delay;
use pg::led::{Led, LEDS};
use f3::peripheral;

struct LedPattern<'a> {
    led: &'a Led,
    step: u8
}

impl<'a> LedPattern<'a> {
    fn new(led: &'a Led, step: u8) -> LedPattern {
        LedPattern { led: led, step: step }
    }

    fn tick(&self, t: u8) {
        let diff = ((self.step * 2) as i8) - (t as i8);
        match diff {
            - 2 | - 1 | 0 | 14 => self.led.on(),
            _ => self.led.off(),
        };
    }
}
const PATTERNS: [LedPattern<'static>; 8] = [
    LedPattern { led: &LEDS[0], step: 0 },
    LedPattern { led: &LEDS[1], step: 1 },
    LedPattern { led: &LEDS[2], step: 2 },
    LedPattern { led: &LEDS[3], step: 3 },
    LedPattern { led: &LEDS[4], step: 4 },
    LedPattern { led: &LEDS[5], step: 5 },
    LedPattern { led: &LEDS[6], step: 6 },
    LedPattern { led: &LEDS[7], step: 7 },
];

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let half_period = 100;

    loop {
        for i in 0..16 {
            for pattern in PATTERNS.iter() {
                pattern.tick(i);
            }
            delay::ms(half_period);
        }
    }
}
