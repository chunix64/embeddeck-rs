**ESP Display Clock** (`esp-display-clock-rs`)

A clean, modern embedded Rust clock based on `Ratatui` for ESP32 using the ESP-HAL ecosystem, Embassy, and a color SPI display.

<img width="1280" height="960" alt="photo_2026-05-14_09-29-52" src="https://github.com/user-attachments/assets/f49d334e-44d5-4c9e-965e-0d795910060a" />


## Features

- Real-time clock with hardware RTC
- NTP time synchronization over WiFi
- Good TUI using `mipidsi` + `embedded-graphics` + `ratatui` (ongoing)
- Adjustable backlight with smooth PWM control
- Multiple target support:
  - Embedded (ESP32)
  - Desktop (ongoing)
  - More (ongoing)
- Clean architecture with separation of concerns (hardware, models, UI, services)
- Built with async/await using Embassy

## Hardware

Tested with:
- ESP32 (various modules)
- 240×320 ST7789 SPI IPS display
- Backlight connected to a PWM-capable GPIO

Default pinout (defined in `src/hardware/board.rs`):

| Function     | GPIO |
|--------------|------|
| SPI SCK      | 18   |
| SPI MOSI     | 23   |
| Display DC   | 2    |
| Display CS   | 5    |
| Display RST  | 4    |
| Backlight    | 14   |

You can easily change pins by editing the board configuration.

## Prerequisites
- `rustc`: 1.91+

## Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/chunix64/esp-display-clock-rs.git
cd esp-display-clock-rs
```

### 2. Configure WiFi

Edit `src/main.rs`:

```rust
let wifi_config = WifiConfig {
    ssid: heapless::String::try_from("YOUR_SSID").unwrap(),
    password: heapless::String::try_from("YOUR_PASSWORD").unwrap(),
};
```

### 3. Build & Flash

```bash
# Build and flash
cargo run --release

# Or build
cargo build --release
```

The output binary located at:

```
target/xtensa-esp32-none-elf/release/esp-display-clock-rs
```

## Desktop Development

Great for iterating on the UI quickly:

```bash
cargo run-desktop
```

This runs the same UI code in a terminal using `ratatui`. (ongoing)

## Project Structure

```
src/
├── hardware/          # Board, display, backlight, peripherals, WiFi
├── models/            # Clock, configs
├── services/          # NTP, software services
├── ui/                # Ratatui UI
├── main.rs            # Entry point for esp32/embedded
├── main_desktop.rs    # Entry point for desktop version
└── lib.rs
```

## Roadmap / TODOs

- Multiple UI layout, themes
- Weather integration
- Configuration via web interface or BLE
- Deep sleep / low power modes
- Touch support
- More screens support
- Sync between desktop version and embedded version
- Improve the RTC clock accuracy (currently 5000+ ppm drift)

## Contributing

Contributions are welcome! Feel free to open issues or PRs.

## License

This project is licensed under the MIT License.

## Acknowledgments

- [esp-hal](https://github.com/esp-rs/esp-hal)
- [Embassy](https://github.com/embassy-rs/embassy)
- [mipidsi](https://github.com/almindor/mipidsi)
- [ratatui](https://github.com/ratatui-org/ratatui)
