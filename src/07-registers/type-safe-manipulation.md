# Type safe manipulation

The last register we were working with, `ODR`, had this in its documentation:

> Bits 16:31 Reserved, must be kept at reset value

We are not supposed to write to those bits of the register or Bad Stuff May
Happen.

There's also the fact the registers have different read/write permissions. Some
of them are write only, others can be read and wrote to and there must be others
that are read only.

Finally, directly working with hexadecimal addresses is error prone. You already
saw that trying to access an invalid memory address causes an exception which
disrupts the execution of our program.

Wouldn't it be nice if we had an API to manipulate registers in a "safe" manner?
Ideally, the API should encode these three points I've mentioned: No messing
around with the actual addresses, should respect read/write permissions and
should prevent modification of the reserved parts of a register.

Well, we do! The `pg` crate contains a `peripheral` module that provides such
API.

Each register block is modeled as a `struct` where each field is a register.
Each register is a different newtype over e.g. `u32` and exposes a combination
of the following methods: `read`, `write` or `modify` according to its
read/write permissions. Finally, these methods don't take primitive values like
`u32`, instead they take yet another newtype that can be constructed using the
builder pattern and that prevent the modification of the reserved parts of a
register.

The best way to get familiar with this API is to port our running example to it.

``` rust
#![no_std]

extern crate aux;

fn main() {
    let gpioe = aux::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());
}
```

First thing you notice: There are no magic addresses involved. Instead we use a
more human friendly: `gpioe.bsrr` to refer to the `BSRR` register in the `GPIOE`
register block.

Then we have this `write` method that takes a closure. If the "identity" closure
is used (`|w| w`), this method will set the register to its "reset value", the
value it had right after the microcontroller was powered on / reset. That value
is `0x0` for the `BSRR` register. Since we want to write a non-zero value to the
register, we use builder methods like `bs9` to set (`true`) or `br9` reset
(`false`) some of the bits of the register value.

Let's run this program! There's some interesting stuff we can do *while*
debugging the program.

`gpioe` is a reference to the `GPIOE` register block. `print gpioe` will return
the base address of the register block.

```
$ (gdb) print gpioe
$1 = (stm32f30x::gpioc::RegisterBlock *) 0x48001000
```

But if we instead `print *gpioe`, we'll get a "full view" of the register block.
The value of each of its registers will be printed. I recommend enabling pretty
print (`set print pretty on`) first, though, to make the output more readable.

```
(gdb) print *gpioe
$2 = stm32f30x::gpioc::RegisterBlock {
  moder: stm32f30x::gpioc::MODER {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x55550000
      }
    }
  },
  otyper: stm32f30x::gpioc::OTYPER {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  ospeedr: stm32f30x::gpioc::OSPEEDR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  pupdr: stm32f30x::gpioc::PUPDR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  idr: stm32f30x::gpioc::IDR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0xcc
      }
    }
  },
  odr: stm32f30x::gpioc::ODR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  bsrr: stm32f30x::gpioc::BSRR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  lckr: stm32f30x::gpioc::LCKR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  afrl: stm32f30x::gpioc::AFRL {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  afrh: stm32f30x::gpioc::AFRH {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  brr: stm32f30x::gpioc::BRR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  }
}
```

All these newtypes and closures sound like they'd generate large, bloated
programs but, if you actually compile the program in release mode with
[LTO](https://en.wikipedia.org/wiki/Interprocedural_optimization)
enabled, you'll see that it produces exactly the same instructions that the
"unsafe" version that used `write_volatile` and hexadecimal addresses did!

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/registers

08000374 <_ZN9registers4main17hc95971086f02247dE.llvm.1361E493>:
 8000374:       b580            push    {r7, lr}
 8000376:       f7ff ff41       bl      80001fc <aux::init>
 800037a:       f44f 7100       mov.w   r1, #512        ; 0x200
 800037e:       6181            str     r1, [r0, #24]
 8000380:       f44f 6100       mov.w   r1, #2048       ; 0x800
 8000384:       6181            str     r1, [r0, #24]
 8000386:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 800038a:       6181            str     r1, [r0, #24]
 800038c:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 8000390:       6181            str     r1, [r0, #24]
 8000392:       bd80            pop     {r7, pc}
```

The best part of all this is that I didn't have to write a single line of code
in the `peripheral` module. All was automatically generated from a System View
Description (SVD) file using the [svd2rust] tool. This SVD file is actually an
XML file that microcontroller vendors provide and that contains the register
maps of their microcontrollers. The file contains the layout of register blocks,
its base addresses, the read/write permissions of each register, the layout of
the registers, whether a register has reserved bits and much more information.

[svd2rust]: https://crates.io/crates/svd2rust
