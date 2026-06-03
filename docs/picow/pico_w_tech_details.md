# Raspberry Pi Pico W Technical Details

Based on the official Raspberry Pi documentation, here are the technical specifications and details for the Raspberry Pi Pico W (and Pico WH).

## Microcontroller (RP2040)
* **Processor:** Dual-core ARM Cortex-M0+
* **Clock Speed:** Flexible clock running up to 133 MHz
* **SRAM:** 264 kB
* **USB:** USB 1.1 controller and PHY with device and host support
* **Power Management:** Low-power sleep and dormant modes
* **Timing:** Accurate clock and timer
* **Math capabilities:** Accelerated floating-point libraries
* **Sensors:** On-chip temperature sensor
* **Custom I/O:** 8 Programmable I/O (PIO) state machines for custom peripheral support (can emulate interfaces like SD card and VGA)

## Board Features
* **Flash Memory:** 2 MB of on-board flash memory
* **Programming:** Drag-and-drop programming using mass storage over USB
* **GPIO:** 26 multi-function GPIO pins
* **Debugging:** Dedicated Debug connector (SWD interface)

## Peripheral Interfaces
* **SPI:** 2
* **I2C:** 2
* **UART:** 2
* **ADC:** 3 (12-bit)
* **PWM channels:** 16

## Wireless Connectivity
* **Wi-Fi (802.11n):**
  * Single-band (2.4 GHz)
  * WPA3 (modern security protocol for wireless connectivity)
  * Soft access point (broadcasting a Wi-Fi network, supporting up to four clients)
* **Bluetooth 5.2:**
  * Support for Bluetooth Low Energy (BLE) Central and Peripheral roles
  * Support for Bluetooth Classic

## Board Layout Highlights
* **Power/Data:** Micro USB port located at the top edge of the board.
* **BOOTSEL:** Button located under the USB port and to the left side of the board.
* **Pins:** 40 pins located on the left and right edges of the board (20 on each side).
* **LED:** Connected differently on the W version compared to the non-wireless Pico.

*Source: [Raspberry Pi Microcontrollers Documentation](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html#layout_wireless)*
