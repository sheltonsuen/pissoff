pub trait Delay {
    fn delay_ms(&mut self, ms: u32);

    fn delay_us(&mut self, us: u32);
}
