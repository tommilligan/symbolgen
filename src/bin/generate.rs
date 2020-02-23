use std::fs::File;

use cairo::{Context, Format, ImageSurface, LineCap};
use symbolgen::{Glyph, PVector};

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
