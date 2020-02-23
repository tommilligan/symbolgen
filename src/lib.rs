use alga::linear::EuclideanSpace;
use nalgebra::{base::Vector2, geometry::Point};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub type PVector = Vector2<f64>;

#[derive(Debug)]
pub struct Glyph {
    width: f64,
    height: f64,

    origin: PVector,
    num_lines: i32,
    resolution: f64,
    density: f64,
    symmetry: bool,
    enable_diags: bool,

    lines: Vec<Line>,
    rng: ChaCha8Rng,
    seed: u64,
}

impl Glyph {
    pub fn new(
        width: f64,
        height: f64,
        origin: &PVector,
        resolution: f64,
        density: f64,
        symmetry: bool,
        enable_diags: bool,
        seed: u64,
    ) -> Self {
        let mut new_self = Self {
            width,
            height,
            origin: origin.clone(),
            resolution,
            density,
            symmetry,
            enable_diags,

            num_lines: (density * resolution).floor() as i32,
            lines: Vec::new(),

            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        };
        new_self.generate();
        new_self
    }

    fn generate(&mut self) {
        for _i in 0..self.num_lines {
            let coin_flip = self.rng.gen::<bool>();
            let coin_flip2 = self.rng.gen::<bool>();

            let x1 = ((self.rng.gen::<f64>() * self.resolution)
                / (if self.symmetry { 1.0 } else { 2.0 }))
            .floor();
            let y1 = (self.rng.gen::<f64>() * (self.resolution - 1.0)).ceil();
            let v1 = PVector::new(x1, y1);
            let mut additive = PVector::new(0.0, 0.0);

            if !self.enable_diags {
                if coin_flip {
                    if v1.x == 0.0 {
                        additive += PVector::new(1.0, 0.0);
                    } else if v1.x == (self.resolution - 1.0) {
                        additive -= PVector::new(1.0, 0.0);
                    } else {
                        additive +=
                            PVector::new(self.rng.gen_range::<f64, _, _>(-1.0, 2.0).floor(), 0.0);
                    }
                } else {
                    if v1.y == 0.0 {
                        additive += PVector::new(0.0, 1.0);
                    } else if v1.y == (self.resolution - 1.0) {
                        additive -= PVector::new(0.0, 1.0);
                    } else {
                        additive +=
                            PVector::new(0.0, self.rng.gen_range::<f64, _, _>(-1.0, 2.0).floor());
                    }
                }
            } else {
                if coin_flip {
                    additive +=
                        PVector::new(self.rng.gen_range::<f64, _, _>(-1.0, 2.0).floor(), 0.0);
                };
                if coin_flip2 {
                    additive +=
                        PVector::new(0.0, self.rng.gen_range::<f64, _, _>(-1.0, 2.0).floor());
                };
            }

            let mut v2 = v1.clone() + additive;
            v2 = PVector::new(
                v2.x.max(0.0).min(self.resolution - 1.0).ceil(),
                v2.y.max(0.0).min(self.resolution - 1.0).ceil(),
            );

            let v1_point = Point::from(v1);
            let v2_point = Point::from(v2);

            let mut dupe = false;
            for line in self.lines.iter() {
                let start_point = Point::from(line.start());
                let end_point = Point::from(line.end());
                if (start_point.distance(&v1_point) == 0.0 && end_point.distance(&v2_point) == 0.0)
                    || (start_point.distance(&v2_point) == 0.0
                        && end_point.distance(&v1_point) == 0.0)
                {
                    dupe = true;
                    break;
                }
            }

            if !dupe && v1_point.distance(&v2_point) > 0.0 {
                self.lines.push(Line::new(v1, v2));
            }
        }
    }

    pub fn render(self) -> Vec<Line> {
        if self.symmetry {
            let mut rendered_lines = self.lines.clone();
            for line in self.lines.iter() {
                let x1 = line.start().x;
                let x2 = line.end().x;
                let start = PVector::new(x1 + (self.width / 2.0 - x1) * 2.0, line.start().y);
                let end = PVector::new(x2 + (self.width / 2.0 - x2) * 2.0, line.end().y);
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
    start: PVector,
    end: PVector,
}

impl Line {
    pub fn new(start: PVector, end: PVector) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> PVector {
        self.start
    }

    pub fn end(&self) -> PVector {
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
