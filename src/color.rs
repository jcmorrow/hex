#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub blue: f64,
    pub green: f64,
    pub red: f64,
}

impl Color {
    pub fn ppm(&self) -> String {
        return format!(
            "{} {} {}",
            (clamp(self.red, 0.0, 1.0) * 255.0).round(),
            (clamp(self.green, 0.0, 1.0) * 255.0).round(),
            (clamp(self.blue, 0.0, 1.0) * 255.0).round()
        );
    }

    pub fn red() -> Color {
        Color {
            red: 0.68,
            blue: 0.8,
            green: 0.45,
        }
    }

    pub fn blue() -> Color {
        Color {
            red: 0.10,
            blue: 0.8,
            green: 0.2,
        }
    }
}

pub fn clamp(number: f64, min: f64, max: f64) -> f64 {
    if number > max {
        max
    } else if number < min {
        min
    } else {
        number
    }
}
