# Phase 1: Flash to Pico + HAL Foundation

## Goal
Get a working Rust firmware flashing to the Pico W with a portable HAL trait layer that enables future migration to other microcontrollers.

## Hardware Setup
- Raspberry Pi Pico W (RP2040)
- External LED connected to GPIO 0 (onboard LED requires WiFi driver)
- SWD debug probe (picoprobe, J-Link, etc.) for probe-rs flashing

## Tech Stack (Version-Pinned)
| Crate | Version | Purpose |
|---|---|---|
| `rp2040-hal` | `0.12` | RP2040 hardware abstraction |
| `rp2040-boot2` | `0.3` | Second-stage bootloader |
| `cortex-m` | `0.7.7` | Cortex-M core primitives |
| `cortex-m-rt` | `0.7.5` | Minimal runtime / startup |
| `defmt` | `1.1` | Embedded logging |
| `defmt-rtt` | `1.2` | RTT transport for defmt |
| `panic-probe` | `1.0` | Panic handler (`print-defmt` feature) |
| `embedded-hal` | `1.0` | Standard embedded HAL traits |
| `probe-rs-tools` | `0.31` | Flashing tool (`cargo install --locked`) |
| `flip-link` | `0.1.12` | Stack overflow protection linker |

## HAL Design Philosophy

Two-layer architecture for portability:

1. **`pissoff-hal`** — Pure traits, `#![no_std]`, zero dependencies
   - Defines hardware interfaces: `OutputPin`, `InputPin`, `Delay`
   - Each trait has associated `Error` type
   - Application code depends ONLY on this crate
   
2. **`pissoff-rp2040`** — Concrete RP2040 implementations
   - Wraps `rp2040-hal` types
   - Implements `pissoff-hal` traits
   - Provides `PicoW` board struct with pre-configured peripherals

**Migration path:** Swap `pissoff-rp2040` for `pissoff-stm32` or `pissoff-esp32` in `firmware/Cargo.toml`. Zero application code changes.

## Step-by-Step Implementation

### Step 1: Workspace Setup

**`Cargo.toml`** (workspace root):
```toml
[workspace]
members = [
  "crates/pissoff-hal",
  "crates/pissoff-rp2040",
  "firmware",
]
resolver = "2"

[workspace.dependencies]
rp2040-hal = { version = "0.12", features = ["rt", "critical-section-impl"] }
rp2040-boot2 = "0.3"
cortex-m = "0.7.7"
cortex-m-rt = "0.7.5"
defmt = "1.1"
defmt-rtt = "1.2"
panic-probe = { version = "1.0", features = ["print-defmt"] }
embedded-hal = "1.0"
```

**`.cargo/config.toml`**:
```toml
[build]
target = "thumbv6m-none-eabi"

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-rs run --chip RP2040"
linker = "flip-link"
rustflags = [
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
  "-C", "no-vectorize-loops",
]

[env]
DEFMT_LOG = "debug"
```

**`memory.x`** (linker script):
```ld
MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
```

---

### Step 2: `pissoff-hal` Crate

**Path:** `crates/pissoff-hal/`

**`Cargo.toml`:**
```toml
[package]
name = "pissoff-hal"
version = "0.1.0"
edition = "2021"

[dependencies]
# Zero dependencies - pure traits
```

**`src/lib.rs`:**
```rust
#![no_std]

pub mod gpio;
pub mod delay;
```

**`src/gpio.rs`:**
```rust
/// Output pin trait - can drive a pin high/low
pub trait OutputPin {
    type Error;
    
    /// Set pin to high voltage level
    fn set_high(&mut self) -> Result<(), Self::Error>;
    
    /// Set pin to low voltage level
    fn set_low(&mut self) -> Result<(), Self::Error>;
    
    /// Toggle pin state
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Input pin trait - can read pin state
pub trait InputPin {
    type Error;
    
    /// Check if pin is at high voltage level
    fn is_high(&self) -> Result<bool, Self::Error>;
    
    /// Check if pin is at low voltage level
    fn is_low(&self) -> Result<bool, Self::Error>;
}
```

**`src/delay.rs`:**
```rust
/// Blocking delay trait
pub trait Delay {
    /// Delay for at least `ms` milliseconds
    fn delay_ms(&mut self, ms: u32);
    
    /// Delay for at least `us` microseconds
    fn delay_us(&mut self, us: u32);
}
```

---

### Step 3: `pissoff-rp2040` Crate

**Path:** `crates/pissoff-rp2040/`

**`Cargo.toml`:**
```toml
[package]
name = "pissoff-rp2040"
version = "0.1.0"
edition = "2021"

[dependencies]
pissoff-hal = { path = "../pissoff-hal" }
rp2040-hal = { workspace = true }
rp2040-boot2 = { workspace = true }
embedded-hal = { workspace = true }
cortex-m-rt = { workspace = true }
```

**`build.rs`:**
```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("memory.x");
    
    // Copy memory.x from workspace root to OUT_DIR
    fs::copy("../../memory.x", &dest_path).unwrap();
    
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rerun-if-changed=../../memory.x");
}
```

**`src/lib.rs`:**
```rust
#![no_std]

use rp2040_hal::{self as hal, gpio::Pin, gpio::bank0::*, Clock, Timer};

pub mod gpio;
pub mod delay;
pub mod board;

pub use board::PicoW;
```

**`src/gpio.rs`:**
```rust
use rp2040_hal::gpio::{Pin, PinId, FunctionSioOutput, FunctionSioInput, PullType};
use pissoff_hal::{OutputPin, InputPin};

// Infallible error type for RP2040 GPIO
#[derive(Debug)]
pub struct GpioError;

/// Wrapper for RP2040 output pins
pub struct RpOutputPin<I: PinId> {
    pin: Pin<I, FunctionSioOutput, PullType::None>,
}

impl<I: PinId> RpOutputPin<I> {
    pub fn new(pin: Pin<I, FunctionSioOutput, PullType::None>) -> Self {
        Self { pin }
    }
}

impl<I: PinId> OutputPin for RpOutputPin<I> {
    type Error = GpioError;
    
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high().map_err(|_| GpioError)
    }
    
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low().map_err(|_| GpioError)
    }
    
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle();
        Ok(())
    }
}

/// Wrapper for RP2040 input pins
pub struct RpInputPin<I: PinId> {
    pin: Pin<I, FunctionSioInput, PullType::None>,
}

impl<I: PinId> RpInputPin<I> {
    pub fn new(pin: Pin<I, FunctionSioInput, PullType::None>) -> Self {
        Self { pin }
    }
}

impl<I: PinId> InputPin for RpInputPin<I> {
    type Error = GpioError;
    
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_high().into())
    }
    
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_low().into())
    }
}
```

**`src/delay.rs`:**
```rust
use rp2040_hal::Timer;
use pissoff_hal::Delay;

/// RP2040 timer wrapper
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
        self.timer.delay_ms(ms);
    }
    
    fn delay_us(&mut self, us: u32) {
        self.timer.delay_us(us);
    }
}
```

**`src/board.rs`:**
```rust
use hal::pac;
use hal::{self as hal, Clock, Watchdog, Sio, Timer};
use rp2040_hal::gpio::Pins;

use crate::delay::RpDelay;
use crate::gpio::{RpOutputPin, RpInputPin};

/// Pico W board with pre-configured peripherals
pub struct PicoW {
    pub pins: hal::gpio::Pins,
    pub delay: RpDelay,
    pub timer: Timer,
    pub watchdog: Watchdog,
}

impl PicoW {
    /// Initialize the Pico W board and return configured peripherals
    pub fn init() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        
        // Configure clocks
        let clocks = hal::clocks::init_clocks_and_plls(
            rp2040_hal::XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        ).ok().unwrap();
        
        // Initialize SIO for GPIO access
        let sio = Sio::new(pac.SIO);
        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
        
        // Create timer
        let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
        
        Self {
            pins,
            delay: RpDelay::new(timer.clone()),
            timer,
            watchdog,
        }
    }
    
    /// Configure a GPIO pin as output
    pub fn output_pin<I: hal::gpio::PinId>(
        &self,
        pin: hal::gpio::Pin<I, hal::gpio::FunctionNull, hal::gpio::PullType::None>,
    ) -> RpOutputPin<I> {
        RpOutputPin::new(pin.into_push_pull_output())
    }
    
    /// Configure a GPIO pin as input
    pub fn input_pin<I: hal::gpio::PinId>(
        &self,
        pin: hal::gpio::Pin<I, hal::gpio::FunctionNull, hal::gpio::PullType::None>,
    ) -> RpInputPin<I> {
        RpInputPin::new(pin.into_pull_up_input())
    }
}
```

---

### Step 4: `firmware` Binary Crate

**Path:** `firmware/`

**`Cargo.toml`:**
```toml
[package]
name = "firmware"
version = "0.1.0"
edition = "2021"

[dependencies]
pissoff-hal = { path = "../crates/pissoff-hal" }
pissoff-rp2040 = { path = "../crates/pissoff-rp2040" }
cortex-m-rt = { workspace = true }
defmt = { workspace = true }
defmt-rtt = { workspace = true }
panic-probe = { workspace = true }
```

**`build.rs`:**
```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("memory.x");
    
    // Copy memory.x from workspace root to OUT_DIR
    fs::copy("../memory.x", &dest_path).unwrap();
    
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rerun-if-changed=../memory.x");
}
```

**`src/main.rs`:**
```rust
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
    
    // Initialize board
    let board = PicoW::init();
    info!("Board initialized");
    
    // Configure GPIO 0 as output (connect external LED here)
    let mut led = board.output_pin(board.pins.gpio0);
    info!("LED pin configured on GPIO 0");
    
    let mut delay = board.delay;
    
    info!("Entering main loop");
    
    loop {
        // LED ON
        led.set_high().unwrap();
        info!("LED ON");
        delay.delay_ms(500);
        
        // LED OFF
        led.set_low().unwrap();
        info!("LED OFF");
        delay.delay_ms(500);
    }
}
```

---

## File Creation Order

| # | File | Purpose |
|---|---|---|
| 1 | `Cargo.toml` | Workspace root |
| 2 | `.cargo/config.toml` | Build target + runner |
| 3 | `memory.x` | RP2040 memory layout |
| 4 | `crates/pissoff-hal/Cargo.toml` | HAL traits crate |
| 5 | `crates/pissoff-hal/src/lib.rs` | Module declarations |
| 6 | `crates/pissoff-hal/src/gpio.rs` | GPIO trait definitions |
| 7 | `crates/pissoff-hal/src/delay.rs` | Delay trait definitions |
| 8 | `crates/pissoff-rp2040/Cargo.toml` | RP2040 impl crate |
| 9 | `crates/pissoff-rp2040/build.rs` | Copy memory.x |
| 10 | `crates/pissoff-rp2040/src/lib.rs` | Module declarations |
| 11 | `crates/pissoff-rp2040/src/gpio.rs` | GPIO implementations |
| 12 | `crates/pissoff-rp2040/src/delay.rs` | Delay implementation |
| 13 | `crates/pissoff-rp2040/src/board.rs` | PicoW board struct |
| 14 | `firmware/Cargo.toml` | Firmware binary crate |
| 15 | `firmware/build.rs` | Copy memory.x |
| 16 | `firmware/src/main.rs` | LED blink entry point |

---

## Verification Checklist

- [ ] `cargo check` — compiles for `thumbv6m-none-eabi`
- [ ] `cargo build` — produces ELF binary
- [ ] `cargo clippy` — no warnings
- [ ] `cargo run` — flashes to Pico W via probe-rs
- [ ] External LED on GPIO 0 blinks at 1Hz
- [ ] `defmt` logs appear via RTT: "LED ON" / "LED OFF"
- [ ] HAL traits work: firmware uses only `pissoff-hal` trait methods

---

## Next Steps (After Phase 1)

**Phase 2: Core Runtime**
- Preemptive task scheduler
- Task priorities + context switching
- Basic IPC (channels/mutexes)

**Phase 3: Drivers & WiFi**
- Expand HAL: PWM, ADC, I2C, SPI, UART
- CYW43439 WiFi driver integration
- TCP/IP stack (smoltcp)

**Phase 4: OTA Updates**
- Dual-bank flash layout
- Bootloader with rollback
- HTTP firmware update endpoint

---

## Notes

- **Pico W LED quirk:** Onboard LED requires CYW43 WiFi driver. Phase 1 uses external LED on GPIO 0.
- **Memory layout:** 256KB SRAM (not 264KB — 8KB is striped banked RAM)
- **Portability:** Application code (`firmware/src/main.rs`) imports only `pissoff-hal` traits, never `rp2040-hal` directly.
- **defmt:** All logging uses `defmt::info!`, `defmt::error!`, etc. Requires `defmt-rtt` global logger.
