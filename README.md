# Falling Sand Simulator

A physics-based particle simulation built with Rust and the Bevy game engine. This project implements a falling sand game where different materials interact with each other based on their physical properties like density.

![gif loading...](https://cdn.discordapp.com/attachments/1119728697986003004/1348399171794763786/Desktop_2025.03.01_-_02.06.47.01.gif?ex=67cf5216&is=67ce0096&hm=06080c486837dc6c0f74e62fbfce3b746c8cfb4ebe2030cb34d6b2b566721a0b&)

## Features

- Multiple materials with unique behaviors:
  - Sand: Falls and piles up realistically
  - Water: Flows and interacts with other materials
  - Smoke: Rises and dissipates
  - Concrete: Static building material
- Real-time particle simulation
- Density-based material interactions
- Customizable brush size for drawing
- User-friendly sidebar interface

[gif here displaying the material selection and brush size controls]

## Technical Details

- Built with Rust 🦀
- Uses Bevy game engine (v0.15.2) for rendering and game systems
- Efficient grid-based simulation
- Custom material behavior system
- Multi-threaded updates using Rayon

## Controls

- Left-click and drag to place materials
- Select materials from the sidebar
- Adjust brush size using the slider

## Building and Running

1. Ensure you have Rust installed
2. Clone the repository
3. Run with cargo:
```bash
cargo run --release
```

## Project Structure

- `src/`
  - `main.rs` - Application entry point and setup
  - `config.rs` - Configuration constants
  - `materials/` - Material definitions and behaviors
  - `grid/` - Grid system implementation
  - `plugins/` - Bevy plugins for simulation, input, and UI
  - `systems/` - Core simulation systems
  - `utils/` - Helper functions

## Performance

The simulation is optimized for performance, utilizing:
- Efficient grid updates
- Parallel processing for particle updates
- Optimized rendering with texture updates

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
