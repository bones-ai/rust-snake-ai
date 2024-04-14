//! Simulation
//! Responsible for updating the population and viz
//! Handles generations

use macroquad::prelude::*;

use crate::pop::Population;
use crate::viz::Viz;

pub struct Simulation {
    gen_count: usize,
    pop: Population,
    viz: Viz,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            gen_count: 0,
            pop: Population::new(),
            viz: Viz::new(),
        }
    }

    pub fn update(&mut self, is_viz_enabled: bool, is_slow_mode: bool) {
        let games_alive = self.pop.update();
        if games_alive <= 0 {
            self.end_current_genration();
            self.start_new_generation();
        }

        self.viz.update_settings(is_viz_enabled, is_slow_mode);
        self.viz.update();
        self.viz.draw();
    }

    pub fn start_new_generation(&mut self) {
        self.gen_count += 1;
        self.pop.reset();
    }

    pub fn end_current_genration(&mut self) {
        let stats = self.pop.get_gen_summary();
        self.viz.reset(stats, self.gen_count);
    }
}
