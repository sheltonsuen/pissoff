#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use pissoff_hal::{Delay, OutputPin};
use pissoff_rp2040::PicoW;

#[entry]
fn main() -> ! {
    info!("PissOS Phase 1: Starting up...");

    let board = PicoW::init();
    info!("Board initialized");

    let mut led = PicoW::output_pin(board.pins.gpio0);
    info!("LED pin configured on GPIO 0");

    let mut delay = board.delay;

    info!("Entering main loop");

    loop {
        led.set_high().unwrap();
        info!("LED ON");
        delay.delay_ms(500);

        led.set_low().unwrap();
        info!("LED OFF");
        delay.delay_ms(500);
    }
}
