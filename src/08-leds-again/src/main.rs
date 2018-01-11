#![no_std]

extern crate aux;

fn main() {
    let (gpioe, rcc) = aux::init();

    // TODO initialize GPIOE

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8()
            .set_bit()
            .odr9()
            .set_bit()
            .odr10()
            .set_bit()
            .odr11()
            .set_bit()
            .odr12()
            .set_bit()
            .odr13()
            .set_bit()
            .odr14()
            .set_bit()
            .odr15()
            .set_bit()
    });
}
