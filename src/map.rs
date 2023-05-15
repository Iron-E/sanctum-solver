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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Map<A>
where
	A: AsRef<[Tile]>,
{
	pub name: String,
	pub tileset: Tileset<A>,
}
