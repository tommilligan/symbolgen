//! Genarate a series of glyphs or symbols.
//!
//! With thanks to v3ga. Based on their implementation at:
//! https://github.com/v3ga/Workshop_Processing_Axidraw_Stereolux_2019/blob/cdf0a7fdec7ea5d4f6f2ee72694661aad6278bbf/axidraw_grid/GridCellRenderAntoine.pde#L1
#![deny(clippy::all)]

use std::f64::EPSILON;

use nalgebra::{
    base::{dimension::U2, Vector2},
    geometry::Point as PointN,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub type Point = PointN<f64, U2>;
pub type Vector = Vector2<f64>;

#[derive(PartialEq)]
pub enum Duplicates {
    Yes,
    No,
}

#[derive(Debug)]
pub struct Glyph {
    /// The number of lines generated.
    num_lines: i32,
    /// The numer steps visible along one grid axis.
    resolution: i32,
    /// 1 / resolution
    step: f64,
    /// The number of lines to draw per resolution
    density: i32,
    /// Whether to mirror in the y-axis
    symmetry: bool,
    /// Enable diagonal lines
    enable_diags: bool,

    /// Generated lines
    lines: Vec<Line>,
    /// RNG
    rng: ChaCha8Rng,
    /// Seed
    seed: u64,
}

impl Glyph {
    pub fn new(
        resolution: i32,
        density: i32,
        symmetry: bool,
        enable_diags: bool,
        seed: u64,
    ) -> Self {
        let mut new_self = Self {
            resolution,
            step: 1.0 / (resolution - 1) as f64,
            density,
            symmetry,
            enable_diags,

            num_lines: density * resolution,
            lines: Vec::new(),

            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        };
        new_self.generate();
        new_self
    }

    /// Generate a random x coordinate
    fn gen_x(&mut self) -> f64 {
        let gen_resolution = (self.resolution as f64) / (if self.symmetry { 1.0 } else { 2.0 });

        let index = self.rng.gen_range::<f64, _, _>(0.0, gen_resolution).floor();
        index / (self.resolution - 1) as f64
    }

    /// Generate a random y coordinate
    fn gen_y(&mut self) -> f64 {
        let index = self
            .rng
            .gen_range::<f64, _, _>(0.0, self.resolution as f64)
            .floor();
        index / (self.resolution - 1) as f64
    }

    /// Generate -1, 0, 1 with equal probability.
    fn gen_adjustment(&mut self) -> f64 {
        self.rng.gen_range(-1, 2) as f64
    }

    fn generate(&mut self) {
        for _i in 0..self.num_lines {
            let coin_flip: bool = self.rng.gen();
            let coin_flip2: bool = self.rng.gen();

            let x1 = self.gen_x();
            let y1 = self.gen_y();

            let p1 = Point::new(x1, y1);
            let mut additive = Vector::new(0.0, 0.0);

            if !self.enable_diags {
                // Either adjust x, or y, orthogonally
                if coin_flip {
                    if p1.x == 0.0 {
                        // If no x addition, add half
                        additive += Vector::new(self.step, 0.0);
                    } else if (p1.x - 1.0).abs() < EPSILON {
                        // If full width, subtract half
                        additive += Vector::new(-self.step, 0.0);
                    } else {
                        // If neighther, randomly adjust by up to one resolution
                        additive += Vector::new(self.gen_adjustment() * self.step, 0.0);
                    }
                } else {
                    // If no x addition, add half
                    if p1.y == 0.0 {
                        additive += Vector::new(0.0, self.step);
                    } else if (p1.y - 1.0).abs() < EPSILON {
                        additive += Vector::new(0.0, -self.step);
                    } else {
                        // If neighther, randomly adjust by up to one resolution
                        additive += Vector::new(0.0, self.gen_adjustment() * self.step);
                    }
                }
            } else {
                // If we have diagonals, adjust x and y independently

                if coin_flip {
                    additive += Vector::new(self.gen_adjustment() * self.step, 0.0);
                };
                if coin_flip2 {
                    additive += Vector::new(0.0, self.gen_adjustment() * self.step);
                };
            }

            let mut p2 = p1 + additive;
            // Clamp to valid adjustment range
            p2 = Point::new(p2.x.max(0.0).min(1.0), p2.y.max(0.0).min(1.0));

            // Check if this line already exists
            let mut dupe = Duplicates::No;
            for line in self.lines.iter() {
                if (line.start() == p1 && line.end() == p2)
                    || (line.start() == p2 && line.end() == p1)
                {
                    dupe = Duplicates::Yes;
                    break;
                }
            }

            // Check the line is valid, continue if not
            if dupe == Duplicates::Yes || p1 == p2 {
                continue;
            }

            self.lines.push(Line::new(p1, p2));
        }
    }

    pub fn render(self) -> Vec<Line> {
        if self.symmetry {
            let mut rendered_lines = self.lines.clone();
            for line in self.lines.iter() {
                let start = Point::new(0.5 + (0.5 - line.start().x), line.start().y);
                let end = Point::new(0.5 + (0.5 - line.end().x), line.end().y);
                rendered_lines.push(Line::new(start, end));
            }
            rendered_lines
        } else {
            if self.symmetry {};
            self.lines
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
