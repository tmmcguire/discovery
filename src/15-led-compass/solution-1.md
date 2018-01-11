# Solution 1

``` rust
#![deny(unsafe_code)]
#![no_std]

extern crate aux;

use aux::prelude::*;
use aux::{Direction, I16x3};

fn main() {
    let (mut leds, mut lsm303dlhc, mut delay, _itm) = aux::init();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let dir = match (x > 0, y > 0) {
            // Quadrant I
            (true, true) => Direction::SouthEast,
            // Quadrant II
            (false, true) => Direction::NorthEast,
            // Quadrant III
            (false, false) => Direction::NorthWest,
            // Quadrant IV
            (true, false) => Direction::SouthWest,
        };

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(100_u8);
    }
}
```
