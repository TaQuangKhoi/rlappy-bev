# rlappy-bev
The little jumping bird game, but written with Bevy and Rust.

## About
A Flappy Bird clone (called "Rlappy Bird") implemented in Rust using the Bevy game engine. Control a bird that must navigate through pipes without colliding with them or the ground.

## Features
- Simple and addictive gameplay
- Score tracking
- Randomly generated pipe obstacles
- Gravity and jump physics
- Game states: Menu, Playing, and Game Over

## Controls
- **SPACE**: Jump (in-game) / Start game (in menu)
- **R**: Restart game (after game over)

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

## License
Licensed under the Apache License, Version 2.0. See LICENSE file for details.
