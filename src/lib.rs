pub enum Color {
    Hex(u32),
}

pub struct Canvas {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
}

pub struct Rect {
    top_left: (usize, usize),
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(top_left: (usize, usize), width: usize, height: usize) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }
}

impl Canvas {
    pub fn new((width, height): (usize, usize)) -> Self {
        Self {
            pixels: (0..(width * height)).map(|_| 0).collect(),
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels = match color {
            Color::Hex(hex) => self.pixels.iter().map(|_| hex).collect(),
        };
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let hex = match color {
            Color::Hex(hex) => hex,
        };
        let (x, y) = rect.top_left;
        for i in y..(y + rect.height) {
            for j in x..(x + rect.width) {
                self.pixels.get_mut(i * self.width + j).map(|p| *p = hex);
            }
        }
    }

    pub fn pixels_hex(&self) -> Vec<Color> {
        self.pixels.iter().map(|p| Color::Hex(*p)).collect()
    }
}
