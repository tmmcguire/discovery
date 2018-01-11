//! Initialization code

#![no_std]

extern crate f3;

pub use f3::hal::stm32f30x::tim6;
pub use f3::led::Led;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{rcc, Peripherals, TIM6, RCC};

pub fn init() -> (
    [Led; 8],
    &'static rcc::RegisterBlock,
    &'static tim6::RegisterBlock,
) {
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constraint();

    let mut gpioe = p.GPIOE.split(&mut rcc.AHB);

    let n: Led = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let ne = gpioe
        .PE10
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let e = gpioe
        .PE11
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let se = gpioe
        .PE12
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let s = gpioe
        .PE13
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let sw = gpioe
        .PE14
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let w = gpioe
        .PE15
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let nw = gpioe
        .PE8
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();

    let leds = [n, ne, e, se, s, sw, w, nw];

    (leds, unsafe { &*RCC::ptr() }, unsafe { &*TIM6::ptr() })
}
