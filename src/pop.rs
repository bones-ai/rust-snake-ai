//! Population
//! Handles multiples streams (islands) of neuro-evoloving agents
//! Also responsible for Island Rejuvenation

use std::time::Instant;

use rand::Rng;

use crate::stream::Stream;
use crate::*;

use self::nn::Net;

pub struct Population {
    gen_start_ts: Instant,
    streams: Vec<Stream>,
}

pub struct GenerationSummary {
    pub time_elapsed_secs: f32,
    pub max_score: usize,
    pub best_net: Option<Net>,
}

impl Population {
    pub fn new() -> Self {
        let mut streams = Vec::new();
        for _ in 0..NUM_STREAMS {
            streams.push(Stream::new());
        }

        Self {
            streams,
            gen_start_ts: Instant::now(),
        }
    }

    pub fn update(&mut self) -> usize {
        let mut games_alive = NUM_GAMES_PER_STREAM * NUM_STREAMS;

        for stream in self.streams.iter_mut() {
            games_alive -= stream.update();
        }

        games_alive
    }

    pub fn reset(&mut self) {
        self.gen_start_ts = Instant::now();
        let mut nets = Vec::new();

        // Streams reset
        for stream in self.streams.iter_mut() {
            let best_net = stream.reset();
            nets.push(best_net);
        }

        // No Streams to cross
        if self.streams.len() <= 1 {
            return;
        }

        // Streams crossing
        let mut rng = rand::thread_rng();
        for stream in self.streams.iter_mut() {
            if !stream.is_local_maximum() {
                continue;
            }

            stream.inject(&nets[rng.gen_range(0..nets.len())]);
        }
    }

    pub fn get_gen_summary(&self) -> GenerationSummary {
        let mut max_score = 0;
        let mut best_net = None;

        for stream in self.streams.iter() {
            let (stream_score, stream_net) = stream.get_stream_summary();
            if stream_score > max_score {
                max_score = stream_score;
                best_net = stream_net;
            }
        }

        GenerationSummary {
            max_score,
            time_elapsed_secs: self.gen_start_ts.elapsed().as_secs_f32(),
            best_net,
        }
    }
}
