mod adjacent;
mod coordinate;
mod tile;
mod tileset;

pub use {adjacent::Adjacent, coordinate::Coordinate, tile::Tile, tileset::Tileset};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Map {
	pub name: String,
	pub tileset: Tileset,
}
