/*
This examples toggles both LEDs, LED3 (PC9)
and LED4 (PC8),every second.
*/

#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

extern crate stm32f0xx_hal;
use stm32f0xx_hal::{delay::Delay, prelude::*, stm32};

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

            // Configure PC8 and PC9 as outputs
            let gpioc = p.GPIOC.split(&mut rcc);
            let mut led4 = gpioc.pc8.into_push_pull_output(cs);
            let mut led3 = gpioc.pc9.into_push_pull_output(cs);

            // Get delay provider
            let mut delay = Delay::new(cp.SYST, &rcc);

            // Toggle the LEDs every second
            loop {
                led3.toggle();
                led4.toggle();
                delay.delay_ms(1_000_u16);
            }
        });
    }

    loop {
        continue;
    }
}
