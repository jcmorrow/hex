use super::color::Color;
use std::vec::Vec;

#[derive(Debug)]
pub struct Canvas {
    pub height: i64,
    pub width: i64,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn empty(width: i64, height: i64) -> Canvas {
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for _i in 0..(width * height) {
            pixels.push(Color {
                blue: 0.0,
                green: 0.0,
                red: 0.0,
            });
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn render_ppm(&self) -> String {
        format!(
            "P3
{} {}
255
{}",
            self.width,
            self.height,
            self.pixels_to_ppm()
        )
    }

    fn pixels_to_ppm(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        for i in 0..self.height {
            let mut pixels: Vec<String> = Vec::new();
            for j in 0..self.width {
                pixels.push(self.pixels[(i * self.width + j) as usize].ppm());
            }
            rows.push(Canvas::chunks(pixels.join(" "), 70).join("\n"));
        }

        let mut string = rows.join("\n");
        string.push_str("\n");
        string
    }

    fn chunks(string: String, size: usize) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        if string.len() > size {
            let mut i = size - 1;
            let string_chars: Vec<char> = string.chars().collect();
            while string_chars[i] != ' ' {
                i -= 1;
            }
            let (beginning, end) = string.split_at(i);
            strings.extend(Canvas::chunks(String::from(beginning), size));
            strings.push(String::from(end.trim_matches(' ')));
        } else {
            strings.push(String::from(string.trim_matches(' ')));
        }
        strings
    }
}
