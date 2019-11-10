use super::hex::FractionalHex;
use super::point::Point;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Point,
    pub origin: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Orientation {
    pub f0: f64,
    pub f1: f64,
    pub f2: f64,
    pub f3: f64,

    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
}

impl Layout {
    pub fn default() -> Layout {
        Layout {
            origin: Point { x: 250., y: 250. },
            size: Point { x: 5., y: 5. },
            orientation: Orientation::pointy(),
        }
    }

    pub fn fuzzy_pixel_to_hex(self, p: Point) -> FractionalHex {
        let mut rng = rand::thread_rng();
        let noise_x: f64 = rng.gen();
        let noise_y: f64 = rng.gen();
        let new_x: f64 = p.x + (noise_x * 10. - 5.);
        let new_y: f64 = p.y + (noise_y * 10. - 5.);
        let fuzzy_p = Point { x: new_x, y: new_y };
        self.pixel_to_hex(fuzzy_p)
    }

    pub fn pixel_to_hex(self, p: Point) -> FractionalHex {
        let orientation = self.orientation;
        let point = Point {
            x: (p.x - self.origin.x) / self.size.x,
            y: (p.y - self.origin.y) / self.size.y,
        };
        let q = orientation.b0 * point.x + orientation.b1 * point.y;
        let r = orientation.b2 * point.x + orientation.b3 * point.y;

        FractionalHex { q, r, s: -q - r }
    }
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
