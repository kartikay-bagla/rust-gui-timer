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
- Always-on-top toggle (pin button in top-left corner)
- Custom close button

## Usage

1. Enter the number of minutes in the input field
2. Press Enter or click "Start"
3. The timer counts down in MM:SS format
4. When finished, the window flashes red
5. Click "Reset" to start over

## Controls

- **Drag**: Click and drag the top bar area to move the window
- **Pin (Always on Top)**: Click the circle button in the top-left corner to toggle always-on-top mode (filled = pinned)
- **Close**: Click the X button in the top-right corner
- **Info**: Click the i button to view license and GitHub link
- **Start timer**: Enter minutes and press Enter or click Start
- **Cancel**: Click Cancel while timer is running
- **Reset**: Click Reset when timer finishes

## Always on Top

The app includes a built-in always-on-top toggle (circle button in the top-left corner). Click it to pin the window above other windows.

**Alternative method for KDE Plasma** (if the built-in toggle doesn't work):

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

### macOS

Build and install the app bundle:

```bash
# Install cargo-bundle (one-time)
cargo install cargo-bundle

# Build the app bundle
cargo bundle --release

# Copy to Applications
cp -r target/release/bundle/osx/Timer.app /Applications/
```

The app will appear in Launchpad and can be launched from Finder.

**Note**: On first launch, macOS may show a security warning for unsigned apps. Right-click the app and select "Open" to bypass this.

### Linux

```bash
# Build release binary
cargo build --release

# Copy binary and icon
mkdir -p ~/.local/bin
cp target/release/timer-app ~/.local/bin/
cp icon.png ~/.local/bin/timer-app-icon.png

# Install desktop entry
cp timer-app.desktop ~/.local/share/applications/
chmod +x ~/.local/share/applications/timer-app.desktop
```

The app will appear in your application menu/launcher.

## Releasing a New Version

1. Update version in `Cargo.toml` (both `[package]` and `[package.metadata.bundle]` sections)
2. Update the Changelog section in this README
3. Build and test:
   ```bash
   cargo build --release
   cargo bundle --release  # macOS
   ```
4. Commit changes:
   ```bash
   git add -A
   git commit -m "Release vX.Y.Z: Brief description"
   ```
5. Create and push tag:
   ```bash
   git tag vX.Y.Z
   git push origin main --tags
   ```

## Project Structure

```
desktop-timer/
├── Cargo.toml          # Dependencies and project metadata
├── icon.png            # Application icon (PNG)
├── icons/
│   └── timer-app.icns  # macOS icon format
├── LICENSE             # GPL v3 license
├── README.md           # This file
├── timer-app.desktop   # Desktop entry for Linux launchers
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

## Changelog

### v1.1.0
- Added always-on-top pin button (circle in top-left corner)
- Added macOS app bundle support via cargo-bundle

### v1.0.0
- Initial release

## License

GPL v3 - See [LICENSE](LICENSE) file.
