# The solution

``` rust
#![no_std]

#[macro_use]
extern crate aux;

use aux::prelude::*;

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

fn main() {
    let (i2c1, _delay, mut itm) = aux::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| unsafe {
            w.start()
                .set_bit()
                .sadd1()
                .bits(MAGNETOMETER)
                .rd_wrn()
                .clear_bit()
                .nbytes()
                .bits(1)
                .autoend()
                .clear_bit()
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| unsafe { w.txdata().bits(IRA_REG_M) });

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| unsafe {
            w.start()
                .set_bit()
                .nbytes()
                .bits(1)
                .rd_wrn()
                .set_bit()
                .autoend()
                .set_bit()
        });

        // Wait until we have received the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        i2c1.rxdr.read().rxdata().bits()
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);
}
```
