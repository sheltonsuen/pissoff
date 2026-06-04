#![no_std]

pub mod gpio;
pub mod delay;
pub mod board;

pub use board::PicoW;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
