#![deny(unsafe_code)]
#![no_std]

extern crate aux;

fn main() {
    let y;
    let x = 42;
    y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
