#![feature(used)]
#![no_std]

extern crate cast;
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f302x;


use cortex_m::asm;
use core::u32;

#[inline(never)]
fn main() {
    cortex_m::interrupt::free(
        |cs| {
            let mut peripherals = stm32f302x::Peripherals::take().unwrap();
            let mut core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

            // Setup GPIO PA5
            peripherals.RCC.ahbenr.modify(|_, w| w.iopaen().bit(true));
            peripherals.GPIOA.moder.modify(|_, w| unsafe { w.moder5().bits(0b01) });

            let mut counter = 0;
            loop {
                peripherals.GPIOA.bsrr.write(|w| w.bs5().set_bit());
                while counter < 900000 {
                    counter += 1;
                }
                peripherals.GPIOA.bsrr.write(|w| w.br5().set_bit());
                while counter > 0 {
                    counter -= 1;
                }
                //iprintln!(&core_peripherals.ITM.stim[0], "Test string.");
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
