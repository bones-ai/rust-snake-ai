use macroquad::prelude::*;

// Game
pub const GRID_W: i32 = 25;
pub const GRID_H: i32 = 25;

// Sim
pub const NUM_GAMES_PER_STREAM: usize = 1000;
pub const NUM_STREAMS: usize = 1;
pub const NUM_SIM_STEPS: usize = 100;
pub const STREAM_REJUVENATION_PERCENT: f32 = 0.1;
pub const STREAM_LOCAL_MAX_WAIT_SECS: f32 = 90.0;
pub const SIM_SLEEP_MILLIS: u64 = 50;

// Pop
pub const POP_NUM_RETAINED: f32 = 0.01;
pub const POP_NUM_CHILDREN: f32 = 0.5;
pub const POP_NUM_RANDOM: f32 = 0.2;
pub const POP_NUM_RETAINED_MUTATED: f32 = 0.29;

// Viz
pub const VIZ_GRID_W: i32 = 5;
pub const VIZ_GRID_H: i32 = 4;
pub const VIZ_DARK_THEME: bool = true;

// NN
pub const BRAIN_MUTATION_RATE: f32 = 0.1;
pub const BRAIN_MUTATION_VARIATION: f32 = 0.1;
pub const INP_LAYER_SIZE: usize = 12;
pub const HIDDEN_LAYER_SIZE: usize = 8;
pub const OUTPUT_LAYER_SIZE: usize = 4;
