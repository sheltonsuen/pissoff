use rp2040_hal as hal;
use rp2040_hal::gpio::{self, Pin, PinId, FunctionNull, PullType, PullUp, ValidFunction, FunctionSio, SioOutput, SioInput};
use rp2040_hal::clocks;
use rp2040_hal::{Watchdog, Sio, Timer};

use crate::delay::RpDelay;
use crate::gpio::{RpOutputPin, RpInputPin};

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub struct PicoW {
    pub pins: gpio::Pins,
    pub delay: RpDelay,
    pub timer: Timer,
    pub watchdog: Watchdog,
}

impl PicoW {
    pub fn init() -> Self {
        let mut pac = hal::pac::Peripherals::take().unwrap();
        let _core = hal::pac::CorePeripherals::take().unwrap();

        let mut watchdog = Watchdog::new(pac.WATCHDOG);

        let clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        ).ok().unwrap();

        let sio = Sio::new(pac.SIO);
        let pins = gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        Self {
            pins,
            delay: RpDelay::new(timer),
            timer,
            watchdog,
        }
    }

    pub fn output_pin<I: PinId + ValidFunction<FunctionSio<SioOutput>>, P: PullType>(
        pin: Pin<I, FunctionNull, P>,
    ) -> RpOutputPin<I, P> {
        RpOutputPin::new(pin.into_push_pull_output())
    }

    pub fn input_pin<I: PinId + ValidFunction<FunctionSio<SioInput>>, P: PullType>(
        pin: Pin<I, FunctionNull, P>,
    ) -> RpInputPin<I, PullUp> {
        RpInputPin::new(pin.into_pull_up_input())
    }
}
