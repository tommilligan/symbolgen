use std::fs::File;

use alga::linear::EuclideanSpace;
use cairo::{Context, Format, ImageSurface, LineCap};
use nalgebra::{base::Vector2, geometry::Point};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub type PVector = Vector2<f64>;

#[derive(Debug)]
pub struct Glyph {
    width: f64,
    height: f64,
    sw: f64,
    sh: f64,

    origin: PVector,
    num_lines: i32,
    resolution: f64,
    density: f64,
    symmetry: bool,
    enable_diags: bool,

    lines: Vec<Line>,
    stroke_width: f64,
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
            stroke_width: 1.0,

            sw: width / resolution,
            sh: height / resolution,

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

    pub fn update(&mut self) {
        for line in self.lines.iter_mut() {
            line.update();
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

    start_target: PVector,
    end_target: PVector,
}

impl Line {
    pub fn new(start: PVector, end: PVector) -> Self {
        Self {
            start_target: start.clone(),
            end_target: end.clone(),

            start,
            end,
        }
    }

    pub fn start(&self) -> PVector {
        self.start
    }

    pub fn end(&self) -> PVector {
        self.end
    }

    pub fn update(&mut self) {
        self.start = self.start.lerp(&self.start_target, 0.1);
        self.end = self.end.lerp(&self.end_target, 0.1);
    }
}

fn main() {
    let scale = 10.0;
    let spacing = 40.0;
    let line_width = 4.0;
    let canvas_width = 1400;
    let canvas_height = 100;
    let output_path = "out.png";
    let surface = ImageSurface::create(Format::ARgb32, canvas_width, canvas_height)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);

    // paint canvas white
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();
    // work with black objects
    context.set_source_rgb(0.0, 0.0, 0.0);

    let offset_y = spacing;
    for glyph_number in 0..22 {
        let offset_x = spacing + ((scale * 2.0 + spacing) * glyph_number as f64);
        let offset = PVector::new(offset_x, offset_y);

        let glyph = Glyph::new(
            2.0,
            2.0,
            &PVector::new(0.0, 0.0),
            3.0,
            3.0,
            true,
            true,
            glyph_number,
        );

        for line in glyph.render().iter() {
            dbg!(&line);
            let start = (line.start() * scale) + offset;
            let end = (line.end() * scale) + offset;
            context.move_to(start.x, start.y);
            context.line_to(end.x, end.y);
        }
    }
    context.set_line_width(line_width);
    context.set_line_cap(LineCap::Round);
    context.stroke();

    let mut file = File::create(output_path).expect("Couldn't create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn't write to png");
}
