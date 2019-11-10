mod canvas;
mod color;
mod hex;
mod hex_colorer;
mod hex_map;
mod layout;
mod point;

use canvas::Canvas;
use color::Color;
use hex::FractionalHex;
use hex::Hex;
use hex_colorer::color_of_hex;
use hex_map::HexMap;
use layout::Layout;
use point::Point;
use std::fs::File;
use std::io::prelude::*;

const SIZE: i64 = 500;

fn main() -> std::io::Result<()> {
    let filename = "output/test.ppm";
    let mut file = File::create(filename)?;
    let mut canvas = Canvas::empty(SIZE, SIZE);

    let mut hex_map = HexMap::new();

    let layout = Layout::default();
    let pixels: Vec<usize> = (0..canvas.pixels.len()).collect();
    let ps: Vec<Color> = pixels
        .iter()
        .map(|i| {
            color_of_hex(
                &mut hex_map,
                hex_round(layout.perlin_pixel_to_hex(Point {
                    x: (i % SIZE as usize) as f64,
                    y: (i / SIZE as usize) as f64,
                })),
            )
        })
        .collect();

    canvas.pixels = ps;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}

fn hex_round(h: FractionalHex) -> Hex {
    let mut q = h.q.round();
    let mut r = h.r.round();
    let s = h.s.round();
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
