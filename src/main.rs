#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f3xx_hal::stm32;
use stm32f3xx_hal::prelude::*;

use stm32f3xx_hal::serial::Serial;

#[entry]
fn main() -> !{
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();

    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

    let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let usart = Serial::usart1(dp.USART1, (tx,rx), 9600.bps(), clocks, &mut rcc.apb2);
    let (mut tx, _) = usart.split();

    /*for b in b"Hello, world\r\n" {
        block!(tx.write(*b)).unwrap();
    }*/

    tx.bwrite_all(b"Hello, world!\r\n").unwrap();


    loop {

    }
}





