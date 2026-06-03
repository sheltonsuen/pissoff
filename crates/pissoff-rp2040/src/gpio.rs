use rp2040_hal::gpio::{Pin, PinId, FunctionSioOutput, FunctionSioInput, PullType};
use pissoff_hal::{OutputPin, InputPin};
use embedded_hal::digital::{
    OutputPin as EhOutputPin,
    StatefulOutputPin,
    InputPin as EhInputPin,
};

#[derive(Debug)]
pub struct GpioError;

pub struct RpOutputPin<I: PinId, P: PullType> {
    pin: Pin<I, FunctionSioOutput, P>,
}

impl<I: PinId, P: PullType> RpOutputPin<I, P> {
    pub fn new(pin: Pin<I, FunctionSioOutput, P>) -> Self {
        Self { pin }
    }
}

impl<I: PinId, P: PullType> OutputPin for RpOutputPin<I, P> {
    type Error = GpioError;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        EhOutputPin::set_high(&mut self.pin).map_err(|_| GpioError)
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        EhOutputPin::set_low(&mut self.pin).map_err(|_| GpioError)
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        StatefulOutputPin::toggle(&mut self.pin).map_err(|_| GpioError)
    }
}

pub struct RpInputPin<I: PinId, P: PullType> {
    pin: Pin<I, FunctionSioInput, P>,
}

impl<I: PinId, P: PullType> RpInputPin<I, P> {
    pub fn new(pin: Pin<I, FunctionSioInput, P>) -> Self {
        Self { pin }
    }
}

impl<I: PinId, P: PullType> InputPin for RpInputPin<I, P> {
    type Error = GpioError;

    fn is_high(&mut self) -> Result<bool, Self::Error> {
        EhInputPin::is_high(&mut self.pin).map_err(|_| GpioError)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        EhInputPin::is_low(&mut self.pin).map_err(|_| GpioError)
    }
}
