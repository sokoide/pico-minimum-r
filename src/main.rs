#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

mod systimer;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut st = systimer::SystemTimer::new();
    st.init();

    loop {
        info!("* step 1");
        st.delay_ms(1000);
        print_unsafe_counter();

        info!("* step 2");
        st.delay_ms(500);
        print_unsafe_counter();
    }
}

pub fn print_unsafe_counter() {
    unsafe {
        info!("unsafe counter: {}", systimer::COUNTER);
    }
}
