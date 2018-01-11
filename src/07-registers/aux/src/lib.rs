//! Initialization code

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate f3;

use cortex_m::peripheral::ITM;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{gpioc, Peripherals, GPIOE};

#[inline(never)]
pub fn init() -> (ITM, &'static gpioc::RegisterBlock) {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constraint();

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);

    gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE10
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE11
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE12
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE13
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE14
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE15
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    gpioe
        .PE8
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);

    (cp.ITM, unsafe { &*GPIOE::ptr() })
}
