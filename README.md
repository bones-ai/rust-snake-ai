# AI learns to play Snake!
A neural network learns to play snakes

Built with [Rust](https://www.rust-lang.org/) and [Macroquad](https://github.com/not-fl3/macroquad) game engine

![screenshot](/screenshot.png)

# Explaination & Timelapse Video
[![youtube](https://img.youtube.com/vi/YPWy-3CTB-I/0.jpg)](https://youtu.be/YPWy-3CTB-I)

# Controls
- `Tab` - Enable/Disable visualization
- `Space` - Slow down the simulation

# Glossary
- **Neuro-evolution**: A subfield of artificial intelligence and evolutionary computation that uses evolutionary algorithms to evolve artificial neural networks.
- **Genetic Algorithm (GA)**: An optimization algorithm inspired by the process of natural selection and genetics, used to evolve populations of solutions to optimization problems
- **Population**: A collection of individuals (neural networks) that are subject to evolution in a neuro-evolutionary algorithm.
- **Island Model**: A technique in neuro-evolution where multiple separate populations (islands, in code this is called streams) of individuals evolve independently, periodically exchanging individuals between islands to maintain diversity and promote exploration.
- **Mutation**: The process of introducing random changes to the genetic material (weights, biases, or network structure) of individuals in the population. Mutation helps introduce new variations and explore the solution space

# Overview
### Setup
- Every snake has a neural network that acts as its brain.
- The snake can see in 4 direction. It can detect food, wall and itself in these 4 directions. Total number of inputs = `4 * 3 = 12`
- These 12 values are fed as an input to the neural network. The neural network then generates 4 values that indicate the threshold for actions - left, right, bottom and top.
- Every generation has 5 streams (islands) of 1000 snakes each. The snakes in each stream evolve independently of the snakes from other streams
- Occasionally best performing snakes from one stream are injected into another. This technique is called "Island Rejuvenation".
### Algorithm
- The simulation begins at `Generation 0` with 5 streams of games, the individuals in each of these streams have randomly generated neural networks.
- Each step, we update every game i.e pass the vision inputs to the neural network and have it decide on an action to take.
- When the snake collides the walls or when it collides with itself, the game is flagged as completed.
- Aditionally, games are marked complete when the snake isn't able to eat food for a certain number of steps. This is to prevent snakes from performing looping actions indefinitely.
- We update each game in a generation until all the games are complete.
- At the end of each generation, each snake in a stream is ranked based on how it performed. 
- Based on this ranking, parents are chosen for the next batch of snakes. Snake at rank 1 is more probable to be a parent compared to snake at rank 10.
- Here's an example of how the population is distributed throughout every generation:
    1. Top 1% of the snakes are retained for next generation
    2. 50% of the population is newly generated using 2 snakes from the previous generation as their parents
    3. 20% of the snakes are freshly generated with no connection to the past generations
    4. The rest 29% of the population are all mutations of the current best performing snakes
- Once we have a new population, we start a new generation. And the above steps are performed until the simulation is closed manually.
- The above steps result in the snakes fine tuning their strategies which inturn lead to longer snakes.

# Usage
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
