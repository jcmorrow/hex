use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Hex {
    pub q: i64,
    pub r: i64,
    pub s: i64,
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r && self.s == other.s
    }
}
impl Eq for Hex {}

#[derive(Debug, Clone, Copy)]
pub struct FractionalHex {
    pub q: f64,
    pub r: f64,
    pub s: f64,
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
