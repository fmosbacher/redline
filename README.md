# RedLine Graphics Library

Simple graphics library that renders pixel by pixel without external libraries.

Rendering should be done by the user, the lib only compute the values into a `Vec<u32>` (RGBA represented in 32 bits).

## Example

```rust
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
// Use canvas.pixels_raw() to get the pixels as a Vec<u32>
```

![Rendered shapes](./assets/mixes_shapes.png)
