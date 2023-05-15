mod adjacent;
mod coordinate;
mod path;
mod tile;
pub mod tileset;

pub use {adjacent::Adjacent, coordinate::Coordinate, tile::Tile};

use {
	serde::{Deserialize, Serialize},
	tileset::Tileset,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Map {
	pub name: String,
	pub tileset: Tileset,
}
