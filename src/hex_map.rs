use super::hex::Hex;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct HexInfo {
    pub alive: bool,
}

impl HexInfo {
    pub fn rand() -> HexInfo {
        HexInfo {
            alive: rand::random(),
        }
    }
}

pub struct HexMap {
    pub tiles: HashMap<Hex, HexInfo>,
}

impl HexMap {
    pub fn new() -> HexMap {
        let mut tiles = HashMap::new();
        tiles.insert(Hex::origin(), HexInfo::rand());
        tiles.insert(Hex::new(1, 0), HexInfo::rand());
        tiles.insert(Hex::new(0, 1), HexInfo::rand());
        tiles.insert(Hex::new(1, 1), HexInfo::rand());
        tiles.insert(Hex::new(-1, 1), HexInfo::rand());
        tiles.insert(Hex::new(-1, -1), HexInfo::rand());
        HexMap { tiles }
    }

    fn compute_info_for_hex(&mut self, hex: Hex) -> HexInfo {
        let is_axis = hex.r == 0 || hex.s == 0 || hex.q == 0;
        if is_axis {
            let ancestor = self.info(Hex::new(closer_to_zero(hex.q), closer_to_zero(hex.r)));
            let ancestor2 = self.info(Hex::new(
                closer_to_zero(closer_to_zero(hex.q)),
                closer_to_zero(closer_to_zero(hex.r)),
            ));
            if ancestor.alive == ancestor2.alive {
                HexInfo { alive: false }
            } else {
                HexInfo { alive: true }
            }
        } else {
            let ancestor1 = self.info(Hex::new(closer_to_zero(hex.q), hex.r));
            let ancestor2 = self.info(Hex::new(hex.q, closer_to_zero(hex.r)));
            if ancestor1.alive == ancestor2.alive {
                HexInfo { alive: true }
            } else {
                HexInfo { alive: false }
            }
        }
    }

    pub fn info(&mut self, hex: Hex) -> HexInfo {
        if !self.tiles.contains_key(&hex) {
            let result = self.compute_info_for_hex(hex);
            self.tiles.insert(hex, result);
        }
        self.tiles[&hex]
    }
}

fn closer_to_zero(i: i64) -> i64 {
    if i == 0 {
        0
    } else if i < 0 {
        i + 1
    } else {
        i - 1
    }
}
