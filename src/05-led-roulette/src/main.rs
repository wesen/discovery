//#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(fn_traits)]

#[macro_use] extern crate f3;
extern crate pg;
extern crate volatile_register;
extern crate cortex_m;

use pg::delay;
use pg::led::{Led, LEDS};

mod i2c;
mod gpio;
mod base;
mod rcc;
mod exti;
mod syscfg;
mod nvic;

use gpio::Gpioa;
use rcc::Rcc;
use syscfg::Syscfg;
use exti::Exti;
use nvic::Nvic;

// button interaction
fn config_user_button() {
    // power on GPIOA
    rcc_mut().ahbenr.modify(|_, w| w.iopaen(true));

    // configure PA0 as input
    gpioa_mut().moder.modify(|_, w| w.moder0(0b00));
    // 00 = no pull
    // 01 = pull-up
    // 10 = pull-down
    gpioa_mut().pupdr.modify(|_, w| w.pupdr0(0b00));

    // exti0 = gpioa0
    syscfg_mut().exticr1.modify(|_, w| w.exti0(0b000));
    // enable line 0
    exti_mut().imr1.modify(|_, w| w.mr0(true));
    // enable rising edge
    exti_mut().rtsr1.modify(|_, w| w.tr0(true));
    // enable falling edge
    exti_mut().ftsr1.modify(|_, w| w.tr0(true));

    nvic_mut().iser0.modify(|_, w| w.setena(1 << 6));
}

static mut button_handler: Option<*const Fn()> = None;
static mut i: u32 = 0u32;

fn main_button() -> ! {
    config_user_button();

    let f = || {
        exti_mut().pr1.write(|w| w.pr0(true));
        let j;
        unsafe {
            i += 1;
            j = i;
        }
        iprintln!("i: {}", j);
        if j % 2 == 0 {
            LEDS[0].on();
        } else {
            LEDS[0].off();
        }
    };

    unsafe {
        button_handler = Some(&f);
    }

    loop {}
}

// interrupt handler
#[export_name = "_exti0"]
pub extern "C" fn exti0_handler() {
    unsafe {
        if let Some(f) = button_handler {
            (*f).call(());
        }
    }
}

// GPIO mappings
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

pub fn gpioa_mut() -> &'static mut Gpioa {
    unsafe { deref_mut(base::GPIOA) }
}

pub fn rcc() -> &'static Rcc {
    unsafe { deref(base::RCC) }
}

pub fn rcc_mut() -> &'static mut Rcc {
    unsafe { deref_mut(base::RCC) }
}

pub fn syscfg() -> &'static Syscfg {
    unsafe { deref(base::SYSCFG) }
}

pub fn syscfg_mut() -> &'static mut Syscfg {
    unsafe { deref_mut(base::SYSCFG) }
}

pub fn exti() -> &'static Exti {
    unsafe { deref(base::EXTI) }
}

pub fn exti_mut() -> &'static mut Exti {
    unsafe { deref_mut(base::EXTI) }
}

pub fn nvic() -> &'static Nvic {
    unsafe { deref(base::NVIC) }
}

pub fn nvic_mut() -> &'static mut Nvic {
    unsafe { deref_mut(base::NVIC) }
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    main_button();

    loop {}
}
