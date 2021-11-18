#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
	let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

	let mut period = 50_u16;
	let v_period = Volatile::new(&mut period);

	loop {
		for i in 0..8 {
			let next = (i + 1) % 8;
			leds[next].on().ok();
			delay.delay_ms(v_period.read());
			leds[i].off().ok();
			delay.delay_ms(v_period.read());
		}
	}
}
