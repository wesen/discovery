//! Playground

#![feature(asm)]
#![feature(lang_items)]
#![no_std]

#[macro_use]
extern crate f3;

pub use f3::delay;

/// LEDs
pub mod led {
    pub use f3::led::{LEDS, Led};
}

pub use f3::itm;

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    f3::led::init();
    f3::itm::init();
}

