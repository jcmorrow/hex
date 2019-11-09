use super::color::Color;
use super::hex::Hex;
use std::collections::HashMap;

pub fn color_of_hex(hex_colors: &mut HashMap<Hex, Color>, hex: Hex) -> Color {
    hex_colors
        .entry(hex)
        .or_insert(Color {
            red: hex.q as f64 / 20.0_f64,
            blue: hex.r as f64 / 20.0_f64,
            green: hex.s as f64 / 20.0_f64,
        })
        .clone()
}
