# My solution

What solution did you come up with?

Here's mine:

``` rust
#![deny(unsafe_code)]
#![no_std]

extern crate aux;

use aux::prelude::*;
use aux::{Delay, Led};

fn main() {
    let (mut delay, mut leds): (Delay, [Led; 8]) = aux::init();

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    }
}
```

One more thing! Check that your solution also works when compiled in "release"
mode:

```
$ xargo build --target thumbv7em-none-eabihf --release
```

You can test it with this `gdb` command:

```
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
                                                 ~~~~~~~
```

Binary size is something we should always keep an eye on! How big is your
solution? You can check that using the `size` command on the "release" binary:

```
$ arm-none-eabi-size target/thumbv7em-none-eabihf/*/led-roulette
   text    data     bss     dec     hex filename
  20426       0       4   20430    4fce target/thumbv7em-none-eabihf/debug/led-roulette
   2810       0       4    2814     afe target/thumbv7em-none-eabihf/release/led-roulette
```


> **NOTE** The Cargo project is already configured to build the release binary
> using LTO.

Know how to read this output? The `text` section contains the program
instructions. It's around five hundred bytes in my case. On the other hand, the
`data` and `bss` sections contain variables statically allocated in RAM
(`static` variables). I'm not using any so the sizes of these sections are
zero.

One final thing! We have been running our programs from within GDB but our
programs don't depend on GDB at all. You can confirm this be closing both GDB
and OpenOCD and then resetting the board by pressing the black button on the
board. The LED roulette application will run without intervention of GDB.
