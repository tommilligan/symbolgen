use std::fs::File;

use cairo::{Context, Format, ImageSurface, LineCap};
use symbolgen::{Glyph, Vector};

fn main() {
    let columns = 26;
    let rows = 3;

    let scale = 20.0;
    let spacing = 25.0;
    let line_width = 4.0;

    let canvas_width = spacing as i32 + ((scale + spacing) as i32 * columns);
    let canvas_height = spacing as i32 + ((scale + spacing) as i32 * rows);
    let output_path = "out.png";
    let surface = ImageSurface::create(Format::ARgb32, canvas_width, canvas_height)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);

    // paint canvas white
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();
    // work with black objects
    context.set_source_rgb(0.0, 0.0, 0.0);

    for row_number in 0..rows {
        let offset_y = spacing + ((scale + spacing) * row_number as f64);
        for column_number in 0..columns {
            let glyph_number = row_number * columns + column_number;
            let offset_x = spacing + ((scale + spacing) * column_number as f64);
            let offset = Vector::new(offset_x, offset_y);

            let glyph = Glyph::new(3, 3, true, true, glyph_number as u64);

            for line in glyph.render().iter() {
                let start = (line.start() * scale) + offset;
                let end = (line.end() * scale) + offset;
                context.move_to(start.x, start.y);
                context.line_to(end.x, end.y);
            }
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
