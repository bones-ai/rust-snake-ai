//! A simple Neural Network
//! There is no way to train this network
//! It can only be used for neuro-evolution

use rand::Rng;

use crate::*;

#[derive(Clone)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>,
}

#[derive(Clone)]
struct Layer {
    nodes: Vec<Vec<f64>>,
}

impl Net {
    pub fn new() -> Self {
        let layer_sizes = vec![
            INP_LAYER_SIZE,
            HIDDEN_LAYER_SIZE,
            // HIDDEN_LAYER_SIZE,
            OUTPUT_LAYER_SIZE,
        ];

        if layer_sizes.len() < 2 {
            panic!("Need at least 2 layers");
        }
        for &size in layer_sizes.iter() {
            if size < 1 {
                panic!("Empty layers not allowed");
            }
        }

        let mut layers = Vec::new();
        let first_layer_size = *layer_sizes.first().unwrap();
        let mut prev_layer_size = first_layer_size;

        for &layer_size in layer_sizes[1..].iter() {
            layers.push(Layer::new(layer_size, prev_layer_size));
            prev_layer_size = layer_size;
        }

        Self {
            layers,
            n_inputs: first_layer_size,
        }
    }

    pub fn merge(&self, other: &Net) -> Self {
        assert_eq!(self.layers.len(), other.layers.len());

        let mut merged_layers = Vec::new();
        for i in 0..self.layers.len() {
            let merged_layer = &self.layers[i].merge(&other.layers[i]);
            merged_layers.push(merged_layer.clone());
        }

        Net {
            layers: merged_layers,
            n_inputs: self.n_inputs,
        }
    }

    pub fn predict(&self, inputs: &Vec<f64>) -> Vec<Vec<f64>> {
        if inputs.len() != self.n_inputs {
            panic!(
                "Bad input size, expected {:?} but got {:?}",
                self.n_inputs,
                inputs.len()
            );
        }

        let mut outputs = Vec::new();
        outputs.push(inputs.clone());
        for (layer_index, layer) in self.layers.iter().enumerate() {
            let layer_results = layer.predict(&outputs[layer_index]);
            outputs.push(layer_results);
        }

        outputs
    }

    pub fn mutate(&mut self) {
        self.layers.iter_mut().for_each(|l| l.mutate());
    }
}

impl Layer {
    fn new(layer_size: usize, prev_layer_size: usize) -> Self {
        let mut nodes: Vec<Vec<f64>> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..layer_size {
            let mut node: Vec<f64> = Vec::new();
            for _ in 0..prev_layer_size + 1 {
                let random_weight: f64 = rng.gen_range(-1.0f64..1.0f64);
                node.push(random_weight);
            }
            nodes.push(node);
        }

        Self { nodes }
    }

    fn merge(&self, other: &Layer) -> Self {
        assert_eq!(self.nodes.len(), other.nodes.len());
        let mut rng = rand::thread_rng();
        let mut nodes: Vec<Vec<f64>> = Vec::new();

        for (node1, node2) in self.nodes.iter().zip(other.nodes.iter()) {
            let mut merged_node = Vec::new();
            for (&weight1, &weight2) in node1.iter().zip(node2.iter()) {
                let selected_weight = if rng.gen::<bool>() { weight1 } else { weight2 };
                merged_node.push(selected_weight);
            }
            nodes.push(merged_node);
        }

        Self { nodes }
    }

    fn predict(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut layer_results = Vec::new();
        for node in self.nodes.iter() {
            layer_results.push(self.sigmoid(self.dot_prod(&node, &inputs)));
        }

        layer_results
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        for n in self.nodes.iter_mut() {
            for val in n.iter_mut() {
                if rng.gen_range(0.0..1.0) >= BRAIN_MUTATION_RATE {
                    continue;
                }

                *val += rng.gen_range(-BRAIN_MUTATION_VARIATION..BRAIN_MUTATION_VARIATION) as f64;
                if *val > 1.0 || *val < -1.0 {
                    let random_weight = rng.gen_range(-1.0f64..1.0f64);
                    *val = random_weight;
                }
            }
        }
    }

    fn dot_prod(&self, node: &Vec<f64>, values: &Vec<f64>) -> f64 {
        let mut it = node.iter();
        let mut total = *it.next().unwrap();
        for (weight, value) in it.zip(values.iter()) {
            total += weight * value;
        }

        total
    }

    fn sigmoid(&self, y: f64) -> f64 {
        1f64 / (1f64 + (-y).exp())
    }
}
