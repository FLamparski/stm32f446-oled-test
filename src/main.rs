#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f4xx_hal as hal;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use embedded_graphics::{image::Image1BPP, prelude::*};

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c
};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        // as per the STM32F446xC/E datasheet page 60.
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4();
        let sda = gpiob.pb9.into_alternate_af4();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

        // Set up the display
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_i2c(i2c).into();
        disp.init().unwrap();
        disp.flush().unwrap();

        // Display the rustacean
        let im = Image1BPP::new(include_bytes!("../resources/rustacean.data"), 128, 64);
        disp.draw(im.into_iter());
        disp.flush().unwrap();

        loop {}
    }

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}