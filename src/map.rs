mod adjacent;
mod build;
mod coordinate;
mod shortest_path;
mod tile;
pub mod tileset;

pub use {
	adjacent::Adjacent, build::Build, coordinate::Coordinate, shortest_path::ShortestPath,
	tile::Tile,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Map {
	pub name: String,
	pub grid: Vec<Vec<Tile>>,
	pub shortest_path_length: Option<Vec<Option<usize>>>,
}
