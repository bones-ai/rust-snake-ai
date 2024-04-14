# AI learns to play Snake!
This is an ant colony simulation, it internally uses kdtree and query caching, it's able to handle about 5k ants on the cpu.

Built with [Rust](https://www.rust-lang.org/) and [Macroquad](https://github.com/not-fl3/macroquad) game engine

![screenshot](/screenshot.png)

# Showcase with timelapse

[![youtube](https://img.youtube.com/vi/98pUSZAM_7M/0.jpg)](https://youtu.be/98pUSZAM_7M)


## Usage
- Clone the repo
```bash
git clone git@github.com:bones-ai/rust-snake-ai.git
cd rust-snake-ai
```
- Run the simulation
```bash
cargo run --release
```

## Configurations
- The project config file is located at `src/configs.rs`
- Disable `VIZ_DARK_THEME` changes the theming
- The streams feature is still experimental. A single stream with 1000 snakes will yield quick results.
