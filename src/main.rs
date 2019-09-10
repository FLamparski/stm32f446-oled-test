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

        // There's a button on PC13. On the Nucleo board, it's pulled up by a 4.7kOhm resistor
        // and therefore is active LOW. There's even a 100nF capacitor for debouncing - nice for us
        // since otherwise we'd have to debounce in software.
        let gpioc = dp.GPIOC.split();
        let btn = gpioc.pc13.into_pull_down_input();

        // Set up the display
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_i2c(i2c).into();
        disp.init().unwrap();
        disp.flush().unwrap();

        // Display the rustacean
        let im = Image1BPP::new(include_bytes!("../resources/rustacean.data"), 128, 64);
        disp.draw(im.into_iter());
        disp.flush().unwrap();

        // Set up state for the loop
        let mut orientation = DisplayRotation::Rotate0;
        let mut was_pressed = btn.is_low();

        // This runs continuously, as fast as possible
        loop {
            // Check if the button has just been pressed.
            // Remember, active low.
            let is_pressed = btn.is_low();
            if !was_pressed && is_pressed {
                // Since the button was pressed, flip the screen upside down
                orientation = get_next_rotation(orientation);
                disp.set_rotation(orientation).unwrap();
                // Now that we've flipped the screen, store the fact that the button is pressed.
                was_pressed = true;
            } else if !is_pressed {
                // If the button is released, confirm this so that next time it's pressed we'll
                // know it's time to flip the screen.
                was_pressed = false;
            }
        }
    }

    loop {}
}

/// Helper function - what rotation flips the screen upside down from
/// the rotation we're in now?
fn get_next_rotation(rotation: DisplayRotation) -> DisplayRotation {
    return match rotation {
        DisplayRotation::Rotate0 => DisplayRotation::Rotate180,
        DisplayRotation::Rotate180 => DisplayRotation::Rotate0,

        // Default branch - if for some reason we end up in one of the portrait modes,
        // reset to 0 degrees landscape. On most SSD1306 displays, this means down is towards
        // the flat flex coming out of the display (and up is towards the breakout board pins).
        _ => DisplayRotation::Rotate0,
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}