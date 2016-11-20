//#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate f3;
extern crate pg;
extern crate volatile_register;

use pg::delay;
use pg::led::{Led, LEDS};

mod i2c;
mod gpio;
mod base;

use gpio::Gpioa;

fn main_println() {
    let half_period = 100;

    iprintln!("Hello world {}", half_period);
    panic!("Hello world");
}

fn main_roulette() {
    let half_period = 100;

    loop {
        for (current, next) in LEDS.iter().zip(LEDS.iter().cycle().skip(1)) {
            next.on();
            delay::ms(half_period);
            current.off();
            delay::ms(half_period);
        }
    }
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}

pub fn gpioe() -> &'static Gpioa {
    unsafe { deref(base::GPIOE) }
}

pub fn gpioe_mut() -> &'static mut Gpioa {
    unsafe { deref_mut(base::GPIOE) }
}

pub fn gpioa() -> &'static Gpioa {
    unsafe { deref(base::GPIOA) }
}

pub fn gpioa_mut() -> &'static Gpioa {
    unsafe { deref_mut(base::GPIOA) }
}

#[no_mangle]
#[inline(never)]
pub fn main_registers() {
    use core::ptr;

    unsafe {
        const GPIOE_BSRR: u32 = 0x4800_1018;

        gpioe_mut().bsrr.write(|w| { w.br9(true) });
        gpioe_mut().bsrr.write(|w| { w.br11(true) });
        gpioe_mut().bsrr.write(|w| { w.br9(false) });
        gpioe_mut().bsrr.write(|w| { w.br11(false) });


        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));

//        ptr::read_volatile(0x4800_1800 as *const u32);
    }

    unsafe {
        const GPIOE_BSRR: u32 = 0x4800_1018;
        const GPIOE_ODR: u32 = 0x4800_1014;

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn on the NORTH LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn on the EAST LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn off the NORTH LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn off the EAST LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    }
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
//    main_println();
//    main_roulette();
    main_registers();

    loop {}
}
