use super::color::Color;
use super::hex::Hex;
use super::hex_map::HexMap;

pub fn color_of_hex(hex_map: &mut HexMap, hex: Hex) -> Color {
    let info = hex_map.info(hex);
    if info.alive {
        Color::red()
    } else {
        Color::blue()
    }
    // hex_colors
    //     .entry(hex)
    //     .or_insert(Color {
    //         red: hex.q as f64 / 20.0_f64,
    //         blue: hex.r as f64 / 20.0_f64,
    //         green: hex.s as f64 / 20.0_f64,
    //     })
    //     .clone()
}
