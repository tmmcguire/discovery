//! Initialization code

#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate f3;

pub use f3::hal::delay::Delay;
pub use f3::hal::prelude;
pub use f3::led::Led;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::Peripherals;

pub fn init() -> (Delay, [Led; 8]) {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constraint();
    let mut rcc = dp.RCC.constraint();

    let clocks = rcc.CFGR.freeze(&mut flash.ACR);

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);
    let delay = Delay::new(cp.SYST, clocks);

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

    (delay, leds)
}
