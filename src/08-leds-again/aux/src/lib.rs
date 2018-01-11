//! Initialization code

#![no_std]

extern crate f3;

use f3::hal::stm32f30x::{gpioc, rcc, Peripherals, GPIOE, RCC};

pub fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (Peripherals::take().unwrap());

    unsafe { (&*GPIOE::ptr(), &*RCC::ptr()) }
}
