//! Initialization code

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use f3::hal::prelude;
pub use f3::hal::serial::Serial;
pub use f3::hal::stm32f30x::{usart1};
pub use f3::hal::time::MonoTimer;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{Peripherals, USART1, ITM};

pub fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM) {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constraint();
    let mut rcc = dp.RCC.constraint();

    let clocks = rcc.CFGR.freeze(&mut flash.ACR);

    let mut gpioa = dp.GPIOA.split(&mut rcc.AHB);

    let tx = gpioa.PA9.as_af7(&mut gpioa.MODER, &mut gpioa.AFRH);
    let rx = gpioa.PA10.as_af7(&mut gpioa.MODER, &mut gpioa.AFRH);

    Serial::usart1(dp.USART1, 115_200.bps(), (tx, rx), &mut rcc.APB2, clocks);

    unsafe { (&mut *(USART1::ptr() as *mut _), MonoTimer::new(cp.DWT, clocks), cp.ITM) }
}
