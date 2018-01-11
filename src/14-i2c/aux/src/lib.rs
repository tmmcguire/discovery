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
use f3::Lsm303dlhc;
use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{I2C1, Peripherals, ITM};

pub fn init() -> (&'static i2c1::RegisterBlock, Delay, ITM) {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constraint();
    let mut rcc = dp.RCC.constraint();

    let clocks = rcc.CFGR.freeze(&mut flash.ACR);

    let mut gpiob = dp.GPIOB.split(&mut rcc.AHB);
    let scl = gpiob.PB6.as_af4(&mut gpiob.MODER, &mut gpiob.AFRL);
    let sda = gpiob.PB7.as_af4(&mut gpiob.MODER, &mut gpiob.AFRL);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.APB1);

    Lsm303dlhc::new(i2c).unwrap();

    let delay = Delay::new(cp.SYST, clocks);

    unsafe { (&mut *(I2C1::ptr() as *mut _), delay, cp.ITM) }
}
