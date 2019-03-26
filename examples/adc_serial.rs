/*
This example initializes a serial uart bus on PA9
and PA10 and configures PA0 to be an analog input.
The main loop reads the values from PA0 and prints
them to the serial bus.
*/
#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;

use crate::hal::{delay::Delay, prelude::*, serial::*, adc::Adc, stm32};
use cortex_m::peripheral::Peripherals;

use core::fmt::Write;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {

            let mut flash = p.FLASH;
            let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut flash);

            let mut delay = Delay::new(cp.SYST, &rcc);

            let gpioa = p.GPIOA.split(&mut rcc);

            let tx = gpioa.pa9.into_alternate_af1(cs);
            let rx = gpioa.pa10.into_alternate_af1(cs);
            let mut up_down = gpioa.pa0.into_analog(cs);

            let mut adc = Adc::new(p.ADC, &mut rcc);
            let mut serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), &mut rcc);

            loop {

                let val: u16 = adc.read(&mut up_down).unwrap();

                // Wait for reception of a single byte
                //let received = nb::block!(serial.read()).unwrap();

                // Send back previously received byte and wait for completion
                serial.write_fmt(format_args!("Up/Down: {}\r\n" ,val));
                delay.delay_ms(200u16);
            }
        });
    }

    loop {
        continue;
    }
}