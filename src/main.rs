mod canvas;
use canvas::Canvas;
use canvas::Color;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

const SIZE: i64 = 500;

fn main() -> std::io::Result<()> {
    let filename = "output/test.ppm";
    let mut file = File::create(filename)?;
    let mut canvas = Canvas::empty(SIZE, SIZE);
    let mut rng = rand::thread_rng();

    let mut hex_colors: HashMap<Hex, Color> = HashMap::new();

    let layout = Layout {
        origin: Point { x: 0., y: 0. },
        size: Point { x: 20., y: 20. },
        orientation: Orientation::pointy(),
    };
    let pixels: Vec<usize> = (0..canvas.pixels.len()).collect();
    let ps: Vec<Color> = pixels
        .iter()
        .map(|i| {
            color_of_hex(
                &mut hex_colors,
                hex_round(pixel_to_hex(
                    layout,
                    Point {
                        x: (i % SIZE as usize) as f64,
                        y: (i / SIZE as usize) as f64,
                    },
                )),
                &mut rng,
            )
        })
        .collect();

    canvas.pixels = ps;
    println!("{:#?}", Hex::new(0, 0));
    println!("Neighbor Up: {:#?}", Hex::new(0, 0) + HEX_DIRECTIONS[1]);
    println!(
        "Pixel to Hex: {:#?}",
        pixel_to_hex(layout, Point { x: 15., y: 15. })
    );
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}

fn color_of_hex(hex_colors: &mut HashMap<Hex, Color>, hex: Hex, rng: &mut ThreadRng) -> Color {
    hex_colors
        .entry(hex)
        .or_insert(Color {
            red: rng.gen(),
            green: rng.gen(),
            blue: rng.gen(),
        })
        .clone()
}

#[derive(Debug, Clone, Copy, Hash)]
struct Hex {
    q: i64,
    r: i64,
    s: i64,
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r && self.s == other.s
    }
}
impl Eq for Hex {}

#[derive(Debug, Clone, Copy)]
struct FractionalHex {
    q: f64,
    r: f64,
    s: f64,
}

impl Hex {
    pub fn new(q: i64, r: i64) -> Hex {
        Hex { q, r, s: -q - r }
    }
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

const HEX_DIRECTIONS: [Hex; 6] = [
    Hex { q: 1, r: 0, s: -1 },
    Hex { q: 0, r: 1, s: -1 },
    Hex { q: -1, r: 1, s: 0 },
    Hex { q: -1, r: 0, s: 1 },
    Hex { q: 0, r: -1, s: 1 },
    Hex { q: 1, r: -1, s: 0 },
];

#[derive(Debug, Clone, Copy)]
struct Orientation {
    f0: f64,
    f1: f64,
    f2: f64,
    f3: f64,

    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
}

impl Orientation {
    fn pointy() -> Orientation {
        Orientation {
            f0: 3_f64.sqrt(),
            f1: 3_f64.sqrt() / 2.,
            f2: 0.,
            f3: 3. / 2.,
            b0: 3_f64.sqrt() / 3.,
            b1: -1. / 3.,
            b2: 0.,
            b3: 2. / 3.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Layout {
    orientation: Orientation,
    size: Point,
    origin: Point,
}

fn pixel_to_hex(layout: Layout, p: Point) -> FractionalHex {
    let orientation = layout.orientation;
    let point = Point {
        x: (p.x - layout.origin.x) / layout.size.x,
        y: (p.y - layout.origin.y) / layout.size.y,
    };
    let q = orientation.b0 * point.x + orientation.b1 * point.y;
    let r = orientation.b2 * point.x + orientation.b3 * point.y;

    FractionalHex {
        q: q,
        r: r,
        s: -q - r,
    }
}

fn hex_round(h: FractionalHex) -> Hex {
    let mut q = h.q.round();
    let mut r = h.r.round();
    let mut s = h.s.round();
    let q_diff = (q - h.q).abs();
    let r_diff = (r - h.r).abs();
    let s_diff = (s - h.s).abs();
    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    }
    Hex::new(q as i64, r as i64)
}
