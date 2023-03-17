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

fn main() {
    let dims = (640, 320);
    let mut canvas = Canvas::new(dims);

    canvas.clear(Color::Hex(0xd1d5dbff));
    canvas.fill_rect(Rect::new((300, 100), 320, 100), Color::Hex(0xf97316ff));
    canvas.fill_circle(Circle::new((200, 200), 150), Color::Hex(0xfacc15ff));
    canvas.line(Line::new((0, 0), dims), Color::Hex(0xef4444ff));
    canvas.line(Line::new((640, 0), (0, 320)), Color::Hex(0xef4444ff));
    canvas.line(Line::new((320, 0), (280, 320)), Color::Hex(0xef4444ff));
    canvas.line(Line::new((640, 140), (0, 140)), Color::Hex(0xef4444ff));
    canvas.line(Line::new((380, 0), (380, 320)), Color::Hex(0xef4444ff));

    write_to_ppm("examples/mixed_shapes.ppm", &canvas.pixels_raw(), dims);
}
