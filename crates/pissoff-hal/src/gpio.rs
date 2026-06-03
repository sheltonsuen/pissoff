pub trait OutputPin {
    type Error;

    fn set_high(&mut self) -> Result<(), Self::Error>;

    fn set_low(&mut self) -> Result<(), Self::Error>;

    fn toggle(&mut self) -> Result<(), Self::Error>;
}

pub trait InputPin {
    type Error;

    fn is_high(&mut self) -> Result<bool, Self::Error>;

    fn is_low(&mut self) -> Result<bool, Self::Error>;
}
