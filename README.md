# rlappy-bev
The little jumping bird game, but written with Bevy and Rust.

## About
A Flappy Bird clone (called "Rlappy Bev") implemented in Rust using the Bevy game engine. Control a bird that must navigate through pipes without colliding with them or the ground.

## Features
- Simple and addictive gameplay
- Score tracking with progressive difficulty
- Randomly generated pipe obstacles
- Gravity and jump physics
- Pause/Resume functionality
- Screenshot capture for sharing
- Progressive speed increase as you score
- Game states: Menu, Playing, Paused, and Game Over
- **Clickable UI buttons** for starting and restarting the game

## Controls
- **Click START button** or **SPACE**: Start game (in menu)
- **SPACE**: Jump (in-game)
- **P**: Pause/Resume game
- **S**: Take a screenshot (saved as `screenshot-YYYYMMDD-HHMMSS.png`)
- **Click RESTART button** or **R**: Restart game (after game over)

## How to Run
Make sure you have Rust installed. Then:

```bash
cargo run --release
```

## Requirements
- Rust 1.70 or higher
- A display server (X11 or Wayland) for graphics

## Building
```bash
cargo build --release
```

## Releases

### Creating a Release
This project uses GitHub Actions to automatically build and release cross-platform binaries.

To create a new release:
1. Go to the Actions tab in GitHub
2. Select "Create Version Tag" workflow
3. Click "Run workflow"
4. Enter the version number (e.g., `v0.1.1`, `v1.0.0`)
5. The workflow will create a tag and trigger the release build

Alternatively, you can create a tag manually:
```bash
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

The release workflow will automatically:
- Build binaries for Windows (.exe), Linux, and macOS
- Package each binary with the required assets (images) into a ZIP file
- Create a GitHub release with the version tag
- Upload all platform-specific ZIP packages to the release

### Downloading Releases
Pre-built packages are available on the [Releases](https://github.com/TaQuangKhoi/rlappy-bev/releases) page.

Each release includes ZIP files for different platforms:
- `rlappy-bev-x86_64-pc-windows-msvc.zip` - Windows 64-bit
- `rlappy-bev-x86_64-unknown-linux-gnu.zip` - Linux 64-bit
- `rlappy-bev-x86_64-apple-darwin.zip` - macOS 64-bit

To run the game:
1. Download the ZIP file for your platform
2. Extract the ZIP file
3. Run the executable (the assets folder must be in the same directory as the executable)

## License
Licensed under the Apache License, Version 2.0. See LICENSE file for details.
