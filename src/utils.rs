use macroquad::color::Color;
use rand::Rng;

use crate::*;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum FourDirs {
    #[default]
    Left,
    Right,
    Bottom,
    Top,
}

pub fn map_to_unit_interval(value: f32, range: f32) -> f32 {
    let x_abs = range.abs();
    let clamped_value = value.clamp(-x_abs, x_abs);
    (clamped_value + x_abs) / (2.0 * x_abs)
}

pub fn grid_to_world(x: i32, y: i32, tile_size: f32, scale: f32) -> (f32, f32) {
    (x as f32 * tile_size * scale, y as f32 * tile_size * scale)
}

pub fn color_with_a(color: Color, a: f32) -> Color {
    Color::new(color.r, color.g, color.b, a)
}

pub fn are_colors_equal(c1: Color, c2: Color) -> bool {
    c1.r == c2.r && c1.g == c2.g && c1.b == c2.b
}

impl FourDirs {
    pub fn get_rand_dir() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Bottom,
            _ => Self::Top,
        }
    }

    pub fn get_all_dirs() -> [(i32, i32); 4] {
        [
            Self::Left.value(),
            Self::Right.value(),
            Self::Bottom.value(),
            Self::Top.value(),
        ]
    }

    pub fn get_rand_horizontal() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..2) {
            0 => Self::Left,
            _ => Self::Right,
        }
    }

    pub fn get_rand_vertical() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..2) {
            0 => Self::Top,
            _ => Self::Bottom,
        }
    }

    pub fn value(&self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Bottom => (0, 1),
            Self::Top => (0, -1),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            FourDirs::Left => true,
            FourDirs::Right => true,
            _ => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            FourDirs::Top => true,
            FourDirs::Bottom => true,
            _ => false,
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(1..GRID_W - 1),
            y: rng.gen_range(1..GRID_H - 1),
        }
    }
}

impl Into<Point> for (i32, i32) {
    fn into(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}
