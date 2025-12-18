# Desktop Timer

> **LLM Slop Alert**  
> This is completely LLM generated. I tested and it works on my system, I'm open to fixing
> it or building it for you, but I can't promise support for this code. Use at your own risk.

A minimal, borderless desktop timer application built with Rust and egui.

## Features

- Clean, minimal UI with no window decorations
- Supports decimal minutes (e.g., `1.5` for 90 seconds)
- Visual flashing alert when timer completes
- Draggable from the top title bar region
- Custom close button

## Usage

1. Enter the number of minutes in the input field
2. Press Enter or click "Start"
3. The timer counts down in MM:SS format
4. When finished, the window flashes red
5. Click "Reset" to start over

## Controls

- **Drag**: Click and drag the top bar area to move the window
- **Close**: Click the X button in the top-right corner
- **Info**: Click the i button to view license and GitHub link
- **Start timer**: Enter minutes and press Enter or click Start
- **Cancel**: Click Cancel while timer is running
- **Reset**: Click Reset when timer finishes

## Always on Top (KDE)

To keep the timer above other windows on KDE Plasma:

1. Right-click on the app in the taskbar
2. Select "More Actions" → "Configure Special Window Settings"
3. Click "Add Property" → "Keep Above"
4. Set to "Force" → "Yes"
5. Click "Apply"

## Building

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- On Debian/Ubuntu, you may need:
  ```bash
  sudo apt install build-essential pkg-config libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
  ```

### Debug build

```bash
cargo build
cargo run
```

### Release build

```bash
cargo build --release
```

The optimized binary will be at `target/release/timer-app`.

## Installation

```bash
# Copy binary and icon
mkdir -p ~/.local/bin
cp target/release/timer-app ~/.local/bin/
cp icon.png ~/.local/bin/timer-app-icon.png

# Install desktop entry
cp timer-app.desktop ~/.local/share/applications/
chmod +x ~/.local/share/applications/timer-app.desktop
```

The app will appear in your application menu/launcher.

## Project Structure

```
desktop-timer/
├── Cargo.toml          # Dependencies and project metadata
├── icon.png            # Application icon
├── LICENSE             # GPL v3 license
├── README.md           # This file
├── timer-app.desktop   # Desktop entry for app launchers
└── src/
    └── main.rs         # Application source code
```

## Configuration

All constants are defined at the top of `src/main.rs`:

| Constant | Description |
|----------|-------------|
| `WINDOW_SIZE` | Window dimensions (200x200) |
| `CLOSE_BTN_SIZE` | Close button size |
| `TIMER_FONT_SIZE` | Font size for MM:SS display |
| `FLASH_INTERVAL_MS` | Flash speed when timer finishes |
| `BG_COLOR` | Background color (dark gray) |
| `FLASH_COLOR` | Flash color (red) |

## Dependencies

- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) - egui framework for native apps
- [image](https://github.com/image-rs/image) - Icon loading

## License

GPL v3 - See [LICENSE](LICENSE) file.
