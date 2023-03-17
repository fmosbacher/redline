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

pub struct Circle {
    center: (usize, usize),
    radius: usize,
}

impl Circle {
    pub fn new(center: (usize, usize), radius: usize) -> Self {
        Self { center, radius }
    }
}

pub struct Line {
    p1: (usize, usize),
    p2: (usize, usize),
}

impl Line {
    pub fn new(p1: (usize, usize), p2: (usize, usize)) -> Self {
        Self { p1, p2 }
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
        let color = match color {
            Color::Hex(hex) => hex,
        };
        let (x, y) = rect.top_left;
        for i in y..(y + rect.height).min(self.height) {
            for j in x..(x + rect.width).min(self.width) {
                self.pixels[i * self.width + j] = color;
            }
        }
    }

    pub fn fill_circle(&mut self, circle: Circle, color: Color) {
        let color = match color {
            Color::Hex(hex) => hex,
        };
        let (cx, cy) = circle.center;
        let (x, y) = (
            cx.saturating_sub(circle.radius),
            cy.saturating_sub(circle.radius),
        );
        for i in y..(y + circle.radius * 2).min(self.height) {
            for j in x..(x + circle.radius * 2).min(self.width) {
                let dx = if cx > j { cx - j } else { j - cx };
                let dy = if cy > i { cy - i } else { i - cy };
                if dx.saturating_pow(2) + dy.saturating_pow(2) <= circle.radius.saturating_pow(2) {
                    self.pixels[i * self.width + j] = color;
                }
            }
        }
    }

    pub fn line(&mut self, line: Line, color: Color) {
        let color = match color {
            Color::Hex(hex) => hex,
        };
        let x1 = line.p1.0;
        let x2 = line.p2.0;
        let ((x1, y1), (x2, y2)) = if x1 > x2 {
            (line.p2, line.p1)
        } else {
            (line.p1, line.p2)
        };
        let dx = x2 - x1;
        let dy = y2 as isize - y1 as isize;
        let a = dy as f64 / dx as f64;
        let b = y1 as f64 - a * x1 as f64;
        if dx > dy.abs() as usize {
            for x in x1..x2.min(self.width) {
                let y = (a * x as f64 + b) as usize;
                self.pixels.get_mut(y * self.width + x).map(|p| *p = color);
            }
        } else {
            let range = if y1 > y2 {
                y2..y1.min(self.height)
            } else {
                y1..y2.min(self.height)
            };
            for y in range {
                let x = if a == f64::INFINITY {
                    x1
                } else {
                    ((y as f64 - b) / a) as usize
                };
                self.pixels.get_mut(y * self.width + x).map(|p| *p = color);
            }
        }
    }

    pub fn pixels_raw(&self) -> Vec<u32> {
        self.pixels.clone()
    }
}
