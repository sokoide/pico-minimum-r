//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{pac, sio::Sio};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use rp_pico as bsp;

mod systimer;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut st = systimer::SystemTimer::new();
    st.init();

    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        st.delay_ms(500);
        print_unsafe_counter();

        info!("off!");
        led_pin.set_low().unwrap();
        st.delay_ms(500);
        print_unsafe_counter();
    }
}

pub fn print_unsafe_counter() {
    unsafe {
        info!("unsafe counter: {}", systimer::COUNTER);
    }
}
