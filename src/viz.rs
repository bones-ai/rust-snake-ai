//! Visualization
//! Handles everything drawn on screen

use macroquad::prelude::*;

use std::time::Instant;

use crate::game::Game;
use crate::nn::Net;
use crate::pop::GenerationSummary;
use crate::*;

pub struct Viz {
    games: Vec<Game>,
    sim_start_ts: Instant,
    max_score: usize,
    gen_count: usize,
    best_brain: Option<Net>,

    is_slow_mode: bool,
    is_show_viz: bool,
    colors: Colors,
}

struct Colors {
    bg: Color,
    snake_head: Color,
    snake_body: Color,
    food: Color,
    wall: Color,
    text: Color,
    node_enabled: Color,
    node_disabled: Color,
    node_hidden: Color,
    positive: Color,
    negative: Color,
    disabled: Color,
    opacity: f32,
}

impl Viz {
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
            sim_start_ts: Instant::now(),
            max_score: 0,
            gen_count: 0,
            best_brain: None,
            is_slow_mode: false,
            is_show_viz: false,
            colors: if VIZ_DARK_THEME {
                Colors::dark()
            } else {
                Colors::light()
            },
        }
    }

    pub fn update(&mut self) {
        if !self.is_show_viz {
            return;
        }

        let mut num_completed = 0;
        for g in self.games.iter_mut() {
            if g.is_complete {
                num_completed += 1;
            }

            g.update();
        }

        if num_completed >= self.games.len() {
            self.init_games();
        }
    }

    fn init_games(&mut self) {
        let new_brain = Net::new();
        let brain = match &self.best_brain {
            Some(brain) => brain,
            None => &new_brain,
        };

        let num_games = 100;
        let mut games = Vec::new();
        for _ in 0..num_games {
            games.push(Game::with_brain(brain));
        }
        self.games = games;
    }

    pub fn reset(&mut self, summary: GenerationSummary, gen_count: usize) {
        if summary.max_score > self.max_score {
            self.max_score = summary.max_score;
            self.best_brain = summary.best_net;
            // self.init_games();
        }

        self.gen_count = gen_count;
        self.print_gen_info(summary.max_score);
    }

    pub fn draw(&self) {
        clear_background(self.colors.bg);

        self.draw_stats();
        self.draw_best_games();
        self.draw_net();
    }

    fn draw_best_games(&self) {
        let mut pos_x = 0;
        let mut pos_y = 0;

        if self.games.len() <= 0 || !self.is_show_viz {
            return;
        }

        let grid_zero = [0, 1, VIZ_GRID_W, VIZ_GRID_W + 1];
        let mut best_games = self.games.clone();
        best_games.sort_by(|a, b| a.partial_cmp(b).unwrap());
        best_games.reverse();

        for index in 0..(VIZ_GRID_H * VIZ_GRID_W) {
            if !grid_zero.contains(&(index as i32)) {
                let game = &best_games[index as usize];
                self.draw_game(game, pos_x, pos_y, 1.0);
            }

            pos_x += 1;
            if pos_x >= VIZ_GRID_W {
                pos_x = 0;
                pos_y += 1;
            }
            if pos_y >= VIZ_GRID_H {
                break;
            }
        }

        // Render 0th game
        let game = &best_games[0];
        self.draw_game(game, 0, 0, 1.96);
    }

    fn draw_game(&self, game: &Game, pos_x: i32, pos_y: i32, scale: f32) {
        let padding = 10.0;
        let w = (screen_width() - padding * 2.0) * 0.7;
        let h = (screen_height() - padding * 2.0) * 0.99;
        let sq = w.min(h);
        let tile_size = ((sq / 4.0) / GRID_W as f32) * scale as f32;

        for x in 0..=GRID_W {
            for y in 0..=GRID_H {
                let mut color = self.colors.bg;
                let pt = (x, y).into();

                if game.is_wall(pt) {
                    color = self.colors.wall;
                }
                if game.is_snake_body(pt) {
                    color = self.colors.snake_body;
                }
                if game.head == pt {
                    color = self.colors.snake_head;
                }
                if game.food == pt {
                    color = self.colors.food;
                }

                if game.is_complete {
                    color = color_with_a(color, self.colors.opacity);
                }

                let (tx, ty) =
                    grid_to_world((pos_x * GRID_W) + x, (pos_y * GRID_H) + y, tile_size, 1.0);
                draw_rectangle(tx + padding, ty + padding, tile_size, tile_size, color);
            }
        }

        let (tx, ty) = grid_to_world(
            (pos_x * GRID_W) + 3,
            (pos_y * GRID_H) + GRID_H,
            tile_size,
            1.0,
        );
        draw_text(
            format!("{:?}", game.score()).as_str(),
            tx,
            ty,
            30.0,
            if game.is_complete {
                self.colors.disabled
            } else {
                self.colors.text
            },
        );
    }

    fn draw_net(&self) {
        if self.games.is_empty() || !self.is_show_viz {
            return;
        }

        let padding = 10.0;
        let w = (screen_width() - padding * 2.0) * 0.75 + 50.0;
        let h = screen_height() * 1.00;

        let node_border_color = color_with_a(GRAY, 0.0);
        let node_radius = 25.0;
        let node_border_thickness = 2.0;
        let line_thickness = 3.0;
        let y_padding = 120.0;
        let layer_1_x_padding = 0.0;
        let layer_2_x_padding = 150.0;
        let layer_3_x_padding = 300.0;

        let layer_1_y = self.calculate_circle_positions(INP_LAYER_SIZE, node_radius, h, 15.0);
        let layer_2_y = self.calculate_circle_positions(HIDDEN_LAYER_SIZE, node_radius, h, 15.0);
        let layer_3_y = self.calculate_circle_positions(OUTPUT_LAYER_SIZE, node_radius, h, 15.0);
        let (colors1, colors2, colors3) = self.get_node_colors();

        // Bottom Text
        let bt_x = screen_width() * 0.75;
        draw_text(
            "Input",
            bt_x + 5.0,
            screen_height() * 0.97,
            30.0,
            self.colors.text,
        );
        draw_text(
            "Hidden",
            bt_x + layer_2_x_padding,
            screen_height() * 0.85,
            30.0,
            self.colors.text,
        );
        draw_text(
            "Output",
            bt_x + layer_3_x_padding,
            screen_height() * 0.725,
            30.0,
            self.colors.text,
        );

        // Lines
        for (y1, c1) in layer_1_y.iter().zip(colors1.iter()) {
            for y2 in layer_2_y.iter() {
                let color = self.get_line_color(*c1);
                draw_line(
                    w + layer_1_x_padding,
                    *y1 + y_padding,
                    w + layer_2_x_padding,
                    *y2 + y_padding,
                    line_thickness,
                    color,
                );
            }
        }
        for y2 in layer_2_y.iter() {
            for (y3, c3) in layer_3_y.iter().zip(colors3.iter()) {
                let color = self.get_line_color(*c3);
                draw_line(
                    w + layer_2_x_padding,
                    *y2 + y_padding,
                    w + layer_3_x_padding,
                    *y3 + y_padding,
                    line_thickness,
                    color,
                );
            }
        }

        // Nodes
        for (y, c) in layer_1_y.iter().zip(colors1.iter()) {
            draw_circle(w + layer_1_x_padding, *y + y_padding, node_radius, *c);
            draw_circle_lines(
                w + layer_1_x_padding,
                *y + y_padding,
                node_radius,
                node_border_thickness,
                node_border_color,
            );
        }
        for (y, c) in layer_2_y.iter().zip(colors2.iter()) {
            draw_circle(w + layer_2_x_padding, *y + y_padding, node_radius, *c);
            draw_circle_lines(
                w + layer_2_x_padding,
                *y + y_padding,
                node_radius,
                node_border_thickness,
                node_border_color,
            );
        }
        for ((idx, y), c) in layer_3_y.iter().enumerate().zip(colors3.iter()) {
            let (px, py) = (w + layer_3_x_padding, *y + y_padding);
            draw_circle(px, py, node_radius, *c);
            draw_circle_lines(
                px,
                py,
                node_radius,
                node_border_thickness,
                node_border_color,
            );
            let text = match idx {
                0 => "Left",
                1 => "Right",
                2 => "Bottom",
                _ => "Top",
            };
            let color = if are_colors_equal(*c, self.colors.node_enabled) {
                self.colors.text
            } else {
                self.colors.disabled
            };
            draw_text(text, px + 50.0, py + 5.0, 30.0, color);
        }
    }

    fn draw_stats(&self) {
        let w = screen_width() * 0.78;
        let h = screen_height() * 0.07;

        draw_text(
            format!("Gen: {:?}", self.gen_count).as_str(),
            w,
            h,
            50.0,
            self.colors.text,
        );
        draw_text(
            format!("Score: {:?}", self.max_score).as_str(),
            w,
            h + 40.0,
            50.0,
            self.colors.text,
        );
        draw_text(
            format!("Slow: {:?}", self.is_slow_mode).as_str(),
            w,
            h + 80.0,
            50.0,
            if self.is_slow_mode {
                self.colors.positive
            } else {
                self.colors.negative
            },
        );
        draw_text(
            format!("Viz: {:?}", self.is_show_viz).as_str(),
            w,
            h + 120.0,
            50.0,
            if self.is_show_viz {
                self.colors.positive
            } else {
                self.colors.negative
            },
        );

        if !self.is_show_viz {
            draw_text(
                "[Space] - Slow motion",
                w,
                h + 250.0,
                30.0,
                self.colors.text,
            );
            draw_text("[Tab] - Show Viz", w, h + 280.0, 30.0, self.colors.text);
        }
    }

    fn get_line_color(&self, c1: Color) -> Color {
        let mut output_color;
        if are_colors_equal(c1, self.colors.node_enabled) {
            output_color = self.colors.node_enabled;
            output_color.a = 0.3;
        } else {
            output_color = self.colors.node_disabled;
            output_color.a = 0.1;
        }

        output_color
    }

    pub fn update_settings(&mut self, is_viz_enabled: bool, is_slow_mode: bool) {
        self.is_show_viz = is_viz_enabled;
        self.is_slow_mode = is_slow_mode;
    }

    fn get_node_colors(&self) -> (Vec<Color>, Vec<Color>, Vec<Color>) {
        let mut color_enabled = self.colors.node_enabled;
        let mut color_disabled = self.colors.node_disabled;
        let mut color_hidden = self.colors.node_hidden;

        // TODO remove the resorting
        let mut best_games = self.games.clone();
        best_games.sort_by(|a, b| a.partial_cmp(b).unwrap());
        best_games.reverse();
        let game = &best_games[0];

        if game.is_complete {
            color_enabled = self.colors.disabled;
            color_disabled = self.colors.disabled;
            color_hidden = self.colors.disabled;
        }

        let net_out = game.get_net_output();
        let inputs = net_out[0].clone();
        let hidden = net_out[1].clone();
        let output = net_out[2].clone();

        let mut input_colors = Vec::new();
        for i in inputs.iter() {
            if *i == 0.0 {
                input_colors.push(color_disabled);
            } else if *i > 0.2 {
                input_colors.push(color_enabled);
            } else {
                input_colors.push(color_disabled);
            }
        }

        let mut hidden_colors = Vec::new();
        for i in hidden.iter() {
            let opacity = map_to_unit_interval(*i as f32, 0.5);
            if game.is_complete {
                hidden_colors.push(color_with_a(color_hidden, 1.0));
            } else if opacity.is_finite() {
                hidden_colors.push(color_with_a(color_hidden, opacity));
            } else {
                hidden_colors.push(color_with_a(color_hidden, 0.8));
            }
        }

        let max_index = output
            .iter()
            .enumerate()
            .max_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap();
        let mut dir = match max_index {
            0 => FourDirs::Left,
            1 => FourDirs::Right,
            2 => FourDirs::Bottom,
            _ => FourDirs::Top,
        };

        if game.dir.is_horizontal() {
            if dir.is_horizontal() && game.dir != dir {
                dir = game.dir;
            }
        }
        if game.dir.is_vertical() {
            if dir.is_vertical() && game.dir != dir {
                dir = game.dir;
            }
        }

        let mut output_colors = vec![
            color_disabled,
            color_disabled,
            color_disabled,
            color_disabled,
        ];
        if dir == FourDirs::Left {
            output_colors[0] = color_enabled;
        }
        if dir == FourDirs::Right {
            output_colors[1] = color_enabled;
        }
        if dir == FourDirs::Bottom {
            output_colors[2] = color_enabled;
        }
        if dir == FourDirs::Top {
            output_colors[3] = color_enabled;
        }

        // colors
        (input_colors, hidden_colors, output_colors)
    }

    fn calculate_circle_positions(&self, n: usize, r: f32, h: f32, y: f32) -> Vec<f32> {
        let total_height = n as f32 * (2.0 * r) + (n as f32 - 1.0) * y;
        let top_y = (h - total_height) / 2.0;

        let mut positions = Vec::new();
        for i in 0..n {
            let circle_y = top_y + (2.0 * r + y) * i as f32;
            positions.push(circle_y);
        }

        positions
    }

    fn print_gen_info(&self, gen_max_score: usize) {
        let message = format!(
            "Gen: {}, Max Score: {}, Gen Max: {}, Sim Ts: {:.2?}m",
            self.gen_count,
            self.max_score,
            gen_max_score,
            self.sim_start_ts.elapsed().as_secs_f32() / 60.0,
        );
        println!("{}", message);
    }
}

impl Colors {
    fn dark() -> Self {
        Self {
            bg: Color::from_hex(0x28334f),
            snake_head: Color::from_hex(0xe982f4),
            snake_body: Color::from_hex(0x67dbf8),
            food: Color::from_hex(0x7aed86),
            wall: Color::from_hex(0xadb4bf),
            text: WHITE,
            node_enabled: Color::from_hex(0x7aed86),
            node_disabled: Color::from_hex(0xfb7171),
            node_hidden: SKYBLUE,
            positive: Color::from_hex(0x51de88),
            negative: Color::from_hex(0xfb7171),
            disabled: GRAY,
            opacity: 0.3,
        }
    }

    fn light() -> Self {
        Self {
            bg: WHITE,
            snake_head: BLUE,
            snake_body: GREEN,
            food: RED,
            wall: BROWN,
            text: BLACK,
            node_enabled: GREEN,
            node_disabled: RED,
            node_hidden: SKYBLUE,
            positive: GREEN,
            negative: RED,
            disabled: GRAY,
            opacity: 0.3,
        }
    }
}
