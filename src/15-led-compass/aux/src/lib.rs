//! Initialization code

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use f3::hal::delay::Delay;
pub use f3::hal::prelude;
pub use f3::hal::stm32f30x::i2c1;
pub use f3::led::{Direction, Leds};
pub use f3::lsm303dlhc::I16x3;
use f3::Lsm303dlhc;
use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{Peripherals, ITM};

pub fn init() -> (Leds, Lsm303dlhc, Delay, ITM) {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constraint();
    let mut rcc = dp.RCC.constraint();

    let clocks = rcc.CFGR.freeze(&mut flash.ACR);

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);
    let n = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let ne = gpioe
        .PE10
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let e = gpioe
        .PE11
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let se = gpioe
        .PE12
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let s = gpioe
        .PE13
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let sw = gpioe
        .PE14
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let w = gpioe
        .PE15
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let nw = gpioe
        .PE8
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);

    let leds = Leds::new(n, ne, e, se, s, sw, w, nw);
    let mut gpiob = dp.GPIOB.split(&mut rcc.AHB);
    let scl = gpiob.PB6.as_af4(&mut gpiob.MODER, &mut gpiob.AFRL);
    let sda = gpiob.PB7.as_af4(&mut gpiob.MODER, &mut gpiob.AFRL);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.APB1);

    let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let delay = Delay::new(cp.SYST, clocks);

    (leds, lsm303dlhc, delay, cp.ITM)
}
