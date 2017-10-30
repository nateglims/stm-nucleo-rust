#![feature(used)]
#![no_std]

extern crate cast;
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f302x;


use cortex_m::asm;
use cortex_m::itm::write_str;
use stm32f302x::{GPIOA, RCC, ITM};

#[inline(never)]
fn main() {
    cortex_m::interrupt::free(
        |cs| {
            let gpioa = GPIOA.borrow(cs);
            let rcc = RCC.borrow(cs);
            let itm = ITM.borrow(cs);

            // Setup GPIO PA5
            rcc.ahbenr.modify(|_, w| w.iopaen().bit(true));
            gpioa.moder.modify(|_, w| unsafe { w.moder5().bits(0b01) });

            let mut counter = 0;
            loop {
                gpioa.bsrr.write(|w| w.bs5().set_bit());
                while counter < 363636 {
                    counter += 1;
                }
                gpioa.bsrr.write(|w| w.br5().set_bit());
                while counter > 0 {
                    counter -= 1;
                }
                iprintln!(&itm.stim[0], "Test string.");
            }
        });
}

#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
