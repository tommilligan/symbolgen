//! Genarate a series of glyphs or symbols.
//!
//! With thanks to v3ga. Based on their implementation at:
//! https://github.com/v3ga/Workshop_Processing_Axidraw_Stereolux_2019/blob/cdf0a7fdec7ea5d4f6f2ee72694661aad6278bbf/axidraw_grid/GridCellRenderAntoine.pde#L1
#![deny(clippy::all)]

use std::f64::EPSILON;
use std::str::FromStr;

use nalgebra::{
    base::{dimension::U2, Vector2},
    geometry::Point as PointN,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub type Point = PointN<f64, U2>;
pub type Vector = Vector2<f64>;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symmetry {
    Asymmetric,
    Horizontal,
    Vertical,
    HorizontalVertical,
}

impl FromStr for Symmetry {
    type Err = String;
    fn from_str(symmetry: &str) -> Result<Self, Self::Err> {
        match symmetry {
            "asymmetric" => Ok(Symmetry::Asymmetric),
            "horizontal" => Ok(Symmetry::Horizontal),
            "vertical" => Ok(Symmetry::Vertical),
            "horizontalvertical" => Ok(Symmetry::HorizontalVertical),
            _ => Err(format!("Could not parse symmetry '{}'", symmetry)),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Motif {
    Orthogonal,
    Diagonal,
}

#[derive(Debug)]
pub struct Glyph {
    /// The number of lines generated.
    num_lines: i32,
    /// The numer steps visible along one grid axis.
    resolution: i32,
    /// The number of lines to draw per resolution
    density: i32,
    /// Whether to mirror in the y-axis
    symmetry: Symmetry,
    /// Enable diagonal lines
    motif: Motif,

    /// 1 / resolution
    step: f64,

    /// RNG
    rng: ChaCha8Rng,
    /// Seed
    seed: u64,
}

impl Glyph {
    pub fn new(resolution: i32, density: i32, symmetry: Symmetry, motif: Motif, seed: u64) -> Self {
        Self {
            resolution,
            step: 1.0 / (resolution - 1) as f64,
            density,
            symmetry,
            motif,

            num_lines: density * resolution,

            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        }
    }

    /// Generate a random x coordinate
    fn gen_coordinate(&mut self) -> f64 {
        let index = self
            .rng
            .gen_range::<f64, _, _>(0.0, self.resolution as f64)
            .floor();
        index / (self.resolution - 1) as f64
    }

    fn gen_point(&mut self) -> Point {
        Point::new(self.gen_coordinate(), self.gen_coordinate())
    }

    /// Generate -1, 0, 1 with equal probability.
    fn gen_adjustment(&mut self) -> f64 {
        self.rng.gen_range(-1, 2) as f64
    }

    pub fn generate(&mut self) -> Vec<Line> {
        let mut lines = Vec::new();
        for _i in 0..self.num_lines {
            let coin_flip: bool = self.rng.gen();
            let coin_fliend_point: bool = self.rng.gen();

            // Generate a random point to start the line
            let start_point = self.gen_point();
            // Start with no change at all
            let mut additive = Vector::new(0.0, 0.0);

            if self.motif == Motif::Orthogonal {
                // Either adjust x, or y, orthogonally
                if coin_flip {
                    if start_point.x == 0.0 {
                        // If no x addition, add half
                        additive += Vector::new(self.step, 0.0);
                    } else if (start_point.x - 1.0).abs() < EPSILON {
                        // If full width, subtract half
                        additive += Vector::new(-self.step, 0.0);
                    } else {
                        // If neighther, randomly adjust by up to one resolution
                        additive += Vector::new(self.gen_adjustment() * self.step, 0.0);
                    }
                } else {
                    // If no x addition, add half
                    if start_point.y == 0.0 {
                        additive += Vector::new(0.0, self.step);
                    } else if (start_point.y - 1.0).abs() < EPSILON {
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
                if coin_fliend_point {
                    additive += Vector::new(0.0, self.gen_adjustment() * self.step);
                };
            }

            let mut end_point = start_point + additive;
            // Clamp to valid adjustment range
            end_point = Point::new(end_point.x.max(0.0).min(1.0), end_point.y.max(0.0).min(1.0));

            // Check the line is valid, continue if not
            if start_point == end_point {
                continue;
            }

            lines.push(Line::new(start_point, end_point));
        }

        if self.symmetry == Symmetry::Horizontal || self.symmetry == Symmetry::HorizontalVertical {
            for line in lines.clone().iter() {
                let start = Point::new(0.5 + (0.5 - line.start().x), line.start().y);
                let end = Point::new(0.5 + (0.5 - line.end().x), line.end().y);
                lines.push(Line::new(start, end));
            }
        };

        if self.symmetry == Symmetry::Vertical || self.symmetry == Symmetry::HorizontalVertical {
            for line in lines.clone().iter() {
                let start = Point::new(line.start().x, 0.5 + (0.5 - line.start().y));
                let end = Point::new(line.end().x, 0.5 + (0.5 - line.end().y));
                lines.push(Line::new(start, end));
            }
        };
        lines
    }
}

#[derive(Debug, Clone, PartialEq)]
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
