//! Stream
//! Island of neuro-evolving agents

use std::time::Instant;

use rand::distributions::{Distribution, WeightedIndex};

use crate::game::Game;
use crate::nn::Net;
use crate::*;

pub struct Stream {
    games: Vec<Game>,
    max_score: usize,
    max_score_ts: Instant,
}

impl Stream {
    pub fn new() -> Self {
        let mut games = Vec::new();
        for _ in 0..NUM_GAMES_PER_STREAM {
            games.push(Game::new());
        }

        Self {
            games,
            max_score: 0,
            max_score_ts: Instant::now(),
        }
    }

    pub fn update(&mut self) -> usize {
        let mut games_alive = NUM_GAMES_PER_STREAM;

        for g in self.games.iter_mut() {
            g.update();

            let score = g.score();
            if score > self.max_score {
                self.max_score = score;
                self.max_score_ts = Instant::now();
            }

            if g.is_complete {
                games_alive -= 1;
            }
        }

        NUM_GAMES_PER_STREAM - games_alive
    }

    pub fn is_local_maximum(&self) -> bool {
        self.max_score_ts.elapsed().as_secs_f32() > STREAM_LOCAL_MAX_WAIT_SECS
    }

    pub fn inject(&mut self, net: &Net) {
        let new_game = Game::with_brain(net);
        let num_games = (NUM_GAMES_PER_STREAM as f32 * STREAM_REJUVENATION_PERCENT) as usize;

        self.games.drain(0..num_games);
        for _ in 0..num_games {
            self.games.push(new_game.clone());
        }

        self.max_score = 0;
        self.max_score_ts = Instant::now();
    }

    pub fn get_stream_summary(&self) -> (usize, Option<Net>) {
        let mut max_score = 0;
        let mut best_net = None;

        for g in self.games.iter() {
            let score = g.score();
            if score > max_score {
                max_score = score;
                best_net = Some(g.brain.clone());
            }
        }

        (max_score, best_net)
    }

    pub fn reset(&mut self) -> Net {
        let mut rng = rand::thread_rng();
        let gene_pool = self.generate_gene_pool();
        let mut new_games = Vec::new();

        // Population Distribution
        let num_retained = NUM_GAMES_PER_STREAM as f32 * POP_NUM_RETAINED;
        let num_children = NUM_GAMES_PER_STREAM as f32 * POP_NUM_CHILDREN;
        let num_random = NUM_GAMES_PER_STREAM as f32 * POP_NUM_RANDOM;
        let mut num_retained_mutated = NUM_GAMES_PER_STREAM as f32 * POP_NUM_RETAINED_MUTATED;

        // Retained no mutation
        let mut games_sorted = self.games.clone();
        games_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        games_sorted.reverse();
        for i in 0..num_retained as usize {
            let old_brain = games_sorted[i].brain.clone();
            let mut new_game = Game::new();
            new_game.brain = old_brain;

            new_games.push(new_game);
        }

        // Children
        if let Some(pool) = gene_pool {
            for _ in 0..num_children as i32 {
                let rand_parent_1 = self.games[pool.sample(&mut rng)].clone();
                let rand_parent_2 = self.games[pool.sample(&mut rng)].clone();
                let mut new_brain = rand_parent_1.brain.merge(&rand_parent_2.brain);
                new_brain.mutate();

                let new_game = Game::with_brain(&new_brain);
                new_games.push(new_game);
            }
        } else {
            // TODO: Error, failed to create a gene pool
            num_retained_mutated += num_children;
        }

        // Retained with mutation
        for i in 0..num_retained_mutated as usize {
            let mut old_brain = games_sorted[i].brain.clone();
            let mut new_game = Game::new();
            old_brain.mutate();
            new_game.brain = old_brain;

            new_games.push(new_game);
        }

        // Full random
        for _ in 0..num_random as i32 {
            new_games.push(Game::new());
        }

        self.games = new_games;
        games_sorted[0].brain.clone()
    }

    fn generate_gene_pool(&self) -> Option<WeightedIndex<f32>> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for game in self.games.iter() {
            let fitness = game.fitness();
            if fitness > max_fitness {
                max_fitness = fitness;
            }

            if fitness.is_finite() {
                weights.push(fitness);
            }
        }
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness) * 100.0);

        WeightedIndex::new(&weights).ok()
    }
}
