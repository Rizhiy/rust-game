use std::collections::HashMap;

use glam::IVec2;
use noise::{NoiseFn, OpenSimplex};

#[inline]
fn mix64(mut z: u64) -> u64 {
    z ^= z >> 30;
    z = z.wrapping_mul(0x97B1_5F7C_A0AB_C2E3);
    z ^= z >> 27;
    z = z.wrapping_mul(0x5BAC_125C_58AA_FF94);
    z ^ (z >> 31)
}

#[inline]
fn tile_hash(seed: u64, pos: IVec2, type_constant: u64) -> u64 {
    let xu = pos.x as u32 as u64;
    let yu = pos.y as u32 as u64;
    // Constant here is to remove potential repeating pattern and specialise generation for tiles
    let key = seed ^ (xu << 32) ^ yu ^ type_constant;
    mix64(key)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Water,
    Grass,
}

pub struct GameState {
    seed: u32,
    noise: OpenSimplex,
    scale: f64,
    water_level: f64,
    tiles: HashMap<IVec2, Tile>
}

impl GameState {
    pub fn new(seed: u32) -> Self {
        GameState {
            seed,
            noise: OpenSimplex::new(seed),
            scale: 0.05,
            water_level: -0.2,
            tiles: HashMap::new()
        }
    }

    pub fn tile_at(&mut self, pos: IVec2) -> Tile {
        if self.tiles.contains_key(&pos) {
            return self.tiles[&pos]
        }

        let fx = pos.x as f64 * self.scale;
        let fy = pos.y as f64 * self.scale;

        let base = self.noise.get([fx, fy]);
        // Multiply by 3.0 to increase frequency, then by 0,3 to reduce amplitude
        let detail = self.noise.get([fx * 3.0, fy * 3.0]) * 0.3;
        let v = base + detail;

        let tile = if v < self.water_level {
            Tile::Water
        } else {
            Tile::Grass
        };
        self.tiles.insert(pos, tile);
        tile
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(0)
    }
}
