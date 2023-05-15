mod coordinate;
mod tile;

pub use
{
	coordinate::Coordinate,
	tile::Tile,
};

use
{
	std::collections::HashSet,
	serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Map
{
	pub name: String,
	pub tileset: Vec<Vec<Tile>>,
}

impl Map
{
	/// # Summary
	///
	/// Select all of the [`Tile::Buildable`]s next to [`Tile::Spawn`] points on this [`Map`].
	pub fn entrances(self) -> HashSet<Coordinate>
	{
		let spawn_tiles = self.select(&Tile::Spawn);

		todo!()
	}

	/// # Summary
	///
	/// Return a [set](HashSet) of every specific [`Tile`] in the [`Map`].
	pub fn select(self, tile: &Tile) -> HashSet<Coordinate>
	{
		self.tileset.iter().enumerate().flat_map(|(y, inner)|
			inner.iter().enumerate().filter_map(move |(x, value)|
				if value == tile { Some(Coordinate(y, x)) } else { None }
			)
		).collect()
	}
}

#[cfg(test)]
mod tests
{
	use super::{Coordinate, Map, Tile, Tile::*};

	/// # Summary
	///
	/// A representation of the map _Park_ from _Sanctum 2_.
	const PARK: [[Tile; 16]; 14] = [
		[Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Empty,  Empty, Empty, Empty,  Empty],
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Impass, Impass, Empty,  Empty, Empty, Empty,  Empty],
		[Spawn,  Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Impass, Impass, Empty,  Empty, Empty, Empty,  Empty],
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Impass, Empty],
		[Impass, Impass, Impass, Impass, Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Pass,   Core,   Core,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Pass,   Core,   Core,   Pass,   Empty,  Empty,  Empty,  Impass, Empty, Empty, Empty,  Empty],
		[Impass, Impass, Impass, Impass, Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty],
	];

// 		'entrances':{(1,4):5, (2,4):4, (3,4):5, (4,4):6},
// 		'exits':{(9,4):3, (9,5):2, (9,6):2, (9,7):3,
// 				 (10,8):3, (11,8):2, (12,8):2, (13,8):3}}

	#[test]
	fn test_select()
	{
	}
}
