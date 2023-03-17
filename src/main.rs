use std::{fs::File, io::Write};

use redline::*;

fn write_to_ppm(file_name: &str, pixels: &[u32], dims: (usize, usize)) {
    let mut file = File::create(file_name).unwrap();
    let mut file_contents = String::new();
    file_contents
        .push_str(format!("P3 {width} {height} 255\n", width = dims.0, height = dims.1).as_str());
    let rgb_values: String = pixels
        .iter()
        .map(|p| {
            format!(
                "{r} {g} {b}\n",
                r = p >> 24,
                g = p >> 16 & 0xff,
                b = p >> 8 & 0xff
            )
        })
        .collect();
    file_contents.push_str(rgb_values.as_str());
    file.write_all(file_contents.as_bytes()).unwrap();
}

fn mixed_shapes_example() {
    let dims = (640, 320);
    let mut canvas = Canvas::new(dims);

    canvas.clear(Color::Hex(0xd1d5dbff));
    canvas.fill_rect(Rect::new((300, 100), 320, 100), Color::Hex(0xd946efff));
    canvas.fill_circle(Circle::new((200, 200), 150), Color::Hex(0xfacc1580));
    canvas.line(Line::new((640, 0), (0, 320)), Color::Hex(0xef4444ff));
    canvas.line(Line::new((320, 0), (280, 320)), Color::Hex(0xef4444ff));
    canvas.line(Line::new((1_000, 140), (0, 140)), Color::Hex(0xef4444ff));
    canvas.line(Line::new((380, 0), (380, 320)), Color::Hex(0xef4444ff));
    canvas.fill_triangle(
        Triangle::new((20, 20), (120, 20), (20, 120)),
        Color::Hex(0x22c55eff),
    );
    canvas.line(Line::new((0, 0), dims), Color::Hex(0xef444480));
    canvas.fill_triangle(
        Triangle::new((500, 150), (700, 220), (400, 300)),
        Color::Hex(0x22c55ee0),
    );

    write_to_ppm("examples/mixed_shapes.ppm", &canvas.pixels_raw(), dims);
}

fn circles_example() {
    let dims = (640, 320);
    let mut canvas = Canvas::new(dims);

    canvas.clear(Color::Hex(0xd1d5dbff));
    for y in 0..4 {
        for x in 0..8 {
            canvas.fill_circle(
                Circle::new((x * 640 / 8 + 40, y * 80 + 40), (x + y + 1) * 6),
                Color::Hex(0xd946ef50),
            );
        }
    }

    write_to_ppm("examples/circles.ppm", &canvas.pixels_raw(), dims);
}

fn main() {
    mixed_shapes_example();
    circles_example();
}
