# PissOS — Raspberry Pi Pico W Bare-Metal Runtime

## Project Vision

A minimal bare-metal runtime OS for Raspberry Pi Pico W written in Rust. Designed with portability-first: a clean HAL trait layer allows future migration to other microcontrollers (STM32, ESP32) without changing application code.

**Target Hardware**: RP2040 (dual-core ARM Cortex-M0+ @ 133MHz, 264KB SRAM, 2MB Flash, CYW43439 WiFi)

## Phases

### Phase 1: Flash to Pico + HAL Foundation (Current)
- Minimal Cargo workspace setup
- `pissoff-hal` — portable hardware abstraction **traits** (GPIO, Timer, UART, I2C, SPI, etc.)
- `pissoff-rp2040` — Pico W implementation of HAL traits
- `firmware/` — entry point binary (blink LED to prove flashing works)
- `memory.x` — linker script for RP2040 memory layout
- `.cargo/config.toml` — target config, `probe-rs` as runner
- Verify: `cargo run` flashes and blinks the onboard LED

### Phase 2: Core Runtime
- Preemptive task scheduler built on top of HAL
- Task priorities, context switching, basic IPC (channels/mutexes)
- System tick timer integration

### Phase 3: Drivers & WiFi
- Expand HAL traits and implementations (PWM, ADC, watchdog, flash controller)
- CYW43439 WiFi driver integration
- Minimal TCP/IP stack (smoltcp)
- Basic socket API

### Phase 4: OTA Updates
- WiFi-based firmware update mechanism
- Dual-bank flash layout (active + staging)
- Bootloader with rollback support
- HTTP endpoint to receive firmware images

## Directory Structure

```
pissoff/
├── AGENTS.md
├── Cargo.toml              # workspace root
├── crates/
│   ├── pissoff-hal/        # portable HAL traits (no_std, zero deps)
│   ├── pissoff-rp2040/     # RP2040/Pico W HAL implementation
│   ├── kernel/             # scheduler, task management, IPC (Phase 2)
│   ├── wifi/               # CYW43439 driver + networking (Phase 3)
│   └── ota/                # over-the-air update logic (Phase 4)
├── firmware/               # main firmware binary (entry point)
├── memory.x                # linker script (memory layout)
└── .cargo/
    └── config.toml         # target, runner, build settings
```

## HAL Design Philosophy

The HAL is split into two layers:

1. **`pissoff-hal`** — Pure traits defining hardware interfaces. `no_std`, zero dependencies. This is what application code depends on.
2. **`pissoff-rp2040`** — Concrete implementations for RP2040/Pico W. Depends on `rp-hal` under the hood.

To migrate to a different chip, create a new implementation crate (e.g., `pissoff-stm32`) and swap the dependency in `firmware/Cargo.toml`. No application code changes needed.

Future BSP crates:
- `pissoff-stm32` — STM32 family
- `pissoff-esp32` — ESP32 family

## Tech Stack

- **Language**: Rust (`no_std`, targeting `thumbv6m-none-eabi`)
- **HAL Backend**: `rp-hal` (RP2040 hardware abstraction)
- **WiFi Driver**: `cyw43` crate
- **Networking**: `smoltcp` (TCP/IP stack)
- **Logging**: `defmt` (embedded-friendly formatting)
- **Flashing/Debug**: `probe-rs`
- **Linker**: `flip-link` (stack overflow protection)
- **Build**: Cargo workspace

## Code Conventions

- `#![no_std]` everywhere except test harnesses
- Async/await for I/O-bound operations where appropriate
- Error handling: `Result<T, PissError>` with a unified error enum
- All logging via `defmt::` macros (`defmt::info!`, `defmt::error!`, etc.)
- Traits first: define the interface before the implementation
- Public APIs must have doc comments
- Minimize unsafe blocks; document safety invariants when required

## Build & Development

### Prerequisites
```bash
rustup target add thumbv6m-none-eabi
cargo install probe-rs-tools flip-link
```

### Commands
```bash
# Build firmware
cargo build

# Flash and run on connected Pico W (via probe-rs)
cargo run

# Host-side unit tests (for kernel logic, HAL trait tests)
cargo test --lib

# Typecheck only
cargo check

# Lint
cargo clippy
```

### Hardware Setup (Phase 1)
- Connect Pico W via USB
- Use SWD debug probe (e.g., picoprobe, J-Link) for `probe-rs` flashing
- Alternatively, hold BOOTSEL and drag-and-drop UF2 (manual flash)

## Current Milestone

**Phase 1 — Flash to Pico + HAL Foundation**

Success criteria:
1. `cargo run` flashes firmware to Pico W via `probe-rs`
2. Onboard LED blinks (proves GPIO HAL trait works)
3. HAL traits defined: `GpioPin`, `OutputPin`, `InputPin`, `Delay`
4. RP2040 implementation compiles and links correctly
5. Clean workspace structure ready for Phase 2 additions
