use rp2040_hal::Timer;
use pissoff_hal::Delay;
use embedded_hal::delay::DelayNs;

pub struct RpDelay {
    timer: Timer,
}

impl RpDelay {
    pub fn new(timer: Timer) -> Self {
        Self { timer }
    }
}

impl Delay for RpDelay {
    fn delay_ms(&mut self, ms: u32) {
        DelayNs::delay_ms(&mut self.timer, ms);
    }

    fn delay_us(&mut self, us: u32) {
        DelayNs::delay_us(&mut self.timer, us);
    }
}
