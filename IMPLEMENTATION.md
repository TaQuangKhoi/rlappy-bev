# Implementation Summary

## Overview
This project implements a complete Flappy Bird clone called "Rlappy Bird" using Rust and the Bevy game engine (version 0.14).

## Project Structure

```
rlappy-bev/
├── Cargo.toml          # Project dependencies and configuration
├── src/
│   └── main.rs         # Main game implementation (339 lines)
├── README.md           # User documentation
└── LICENSE             # Apache 2.0 License
```

## Key Components

### Game Architecture
The game is built using Bevy's Entity Component System (ECS) architecture with the following main components:

1. **Components**
   - `Bird`: The player-controlled character with velocity
   - `Pipe`: Obstacle entities
   - `Velocity`: Component for moving entities
   - `StartButton`: UI button component for menu and game over screens

2. **Resources**
   - `Score`: Tracks player's score
   - `PipeSpawnTimer`: Controls pipe generation timing
   - `GameDifficulty`: Tracks speed multiplier and progression

3. **Game States**
   - `Menu`: Initial state, waiting to start
   - `Playing`: Active gameplay
   - `Paused`: Game paused state
   - `GameOver`: End state with score display

### Core Systems

1. **setup**: Initializes the game camera and clickable UI button
2. **button_system**: Handles button interactions (hover, click) for starting and restarting the game
3. **menu_system**: Handles menu state and game start via keyboard
4. **bird_input**: Processes space bar input for jumping
5. **bird_movement**: Applies gravity and updates bird position
6. **spawn_pipes**: Generates pipe obstacles at intervals
7. **pipe_movement**: Scrolls pipes across the screen with progressive speed
8. **check_collisions**: Detects bird-pipe and bird-ground collisions
9. **update_score**: Tracks pipes passed by the bird and increases difficulty
10. **pause_input**: Pauses the game when P is pressed
11. **unpause_system**: Resumes the game from pause
12. **screenshot_input**: Captures a screenshot of the game
13. **game_over_system**: Handles game restart via keyboard and displays UI button

### Game Physics
- **Gravity**: -500.0 units/second²
- **Jump Force**: 300.0 units/second
- **Pipe Speed**: 150.0 units/second (base speed)
- **Pipe Gap**: 200.0 units (vertical space between pipes)
- **Spawn Interval**: 2.0 seconds between pipe pairs
- **Speed Increase**: 5% speed increase per pipe passed (up to 2.5x max speed)

### Visual Elements
- Bird: Animated sprite from bird.png (scaled to ~30x30 pixels)
- Pipes: Green rectangles (60x400 pixels)
- Ground: Green rectangle (1000x50 pixels)
- Score display: White text in top-left corner
- **UI Buttons**: Green clickable buttons with hover effects for START and RESTART

## Technical Details

### Dependencies
- **bevy 0.14**: Game engine
  - Disabled default audio features for headless compatibility
  - Enabled: winit, render, core_pipeline, sprite, text, ui, state, png, x11
- **rand 0.8**: Random number generation for pipe positioning
- **chrono 0.4**: Timestamp generation for screenshot filenames

### Collision Detection
Simple Axis-Aligned Bounding Box (AABB) collision detection:
- Bird vs Pipes: Checks distance between centers
- Bird vs Ground: Y-position threshold
- Bird vs Ceiling: Y-position threshold

### Score Calculation
- Score increments when bird passes a pipe
- Divided by 2 in display (since each gap has 2 pipe entities)

## Building and Running

### Requirements
- Rust 1.70 or higher
- X11 or Wayland display server
- Linux system libraries (automatically handled by Cargo)

### Commands
```bash
# Debug build and run
cargo run

# Release build and run (optimized)
cargo run --release

# Build only
cargo build --release
```

## Game Controls
- **Click START button** or **SPACE**: Start game (in menu)
- **SPACE**: Jump (in-game)
- **P** or **Esc**: Pause/Resume game
- **S**: Take a screenshot (saved to current directory)
- **Click RESTART button** or **R**: Restart game (after game over)

## Future Enhancements
Potential improvements could include:
- Sound effects and background music
- High score persistence
- Particle effects
- Leaderboard system
- Mobile touch controls
- Custom sprite graphics
