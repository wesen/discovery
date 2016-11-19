#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

use pg::delay;
use pg::led::{Led, LEDS};

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

//static static_patterns: [&'static LedPattern; 8] = [
//    LedPattern::new(&LEDS[0], 0),
//    LedPattern::new(&LEDS[1], 1),
//    LedPattern::new(&LEDS[2], 2),
//    LedPattern::new(&LEDS[3], 3),
//    LedPattern::new(&LEDS[4], 4),
//    LedPattern::new(&LEDS[5], 5),
//    LedPattern::new(&LEDS[6], 6),
//    LedPattern::new(&LEDS[7], 7),
//];

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let half_period = 100;

    let patterns: [LedPattern; 8] = [
        LedPattern::new(&LEDS[0], 0),
        LedPattern::new(&LEDS[1], 1),
        LedPattern::new(&LEDS[2], 2),
        LedPattern::new(&LEDS[3], 3),
        LedPattern::new(&LEDS[4], 4),
        LedPattern::new(&LEDS[5], 5),
        LedPattern::new(&LEDS[6], 6),
        LedPattern::new(&LEDS[7], 7),
    ];

    loop {
        for i in 0..16 {
            for pattern in patterns.iter() {
                pattern.tick(i);
            }
            delay::ms(half_period);
        }
    }
}
