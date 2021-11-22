#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, RegisterBlock, ITM};

#[entry]
fn main() -> ! {
	let gpioe = aux7::init().1;
	// turn on the north LED
	gpioe.bsrr.write(|w| w.bs9().set_bit());
	// turn on the east LED
	gpioe.bsrr.write(|w| w.bs11().set_bit());
	// turn off north and then east LED
	gpioe.bsrr.write(|w| w.br9().set_bit());
	gpioe.bsrr.write(|w| w.br11().set_bit());
	loop {}
}
