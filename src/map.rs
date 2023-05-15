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
		self.tileset.iter().enumerate().flat_map(|(y, row)|
			row.iter().enumerate().filter_map(move |(x, value)|
				if value == tile { Some(Coordinate(x, y)) } else { None }
			)
		).collect()
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{Coordinate, Map, Tile, Tile::*},
		std::collections::HashSet,
	};

	/// # Summary
	///
	/// A representation of the map _Park_ from _Sanctum 2_.
	const PARK: [[Tile; 16]; 14] = [
		// 0     1       2       3       4       5       6       7       8       9       10      11      12     13     14      15
		[Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Empty,  Empty, Empty, Empty,  Empty], // 0
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Impass, Impass, Empty,  Empty, Empty, Empty,  Empty], // 1
		[Spawn,  Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Impass, Impass, Empty,  Empty, Empty, Empty,  Empty], // 2
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 3
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 4
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 5
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 6
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 7
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 8
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Impass, Empty], // 9
		[Impass, Impass, Impass, Impass, Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 10
		[Impass, Impass, Impass, Impass, Pass,   Core,   Core,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 11
		[Impass, Impass, Impass, Impass, Pass,   Core,   Core,   Pass,   Empty,  Empty,  Empty,  Impass, Empty, Empty, Empty,  Empty], // 12
		[Impass, Impass, Impass, Impass, Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 13
	];

	#[test]
	fn test_select()
	{
		let park = Map
		{
			name: "Park".into(),
			tileset: PARK.iter().map(|row| row.iter().copied().collect()).collect(),
		};

		assert_eq!(
			park.select(&Tile::Impass),
			[
				(0,0),  (1,0),  (2,0),  (3,0),  (4,0),   (5,0), (6,0), (7,0), (8,0), (9,0), (10,0),
				(9,1),  (10,1),
				(9,2),  (10,2),
				(0,5),  (1,5),  (2,5),  (3,5),
				(0,6),  (1,6),  (2,6),  (3,6),
				(0,7),  (1,7),  (2,7),  (3,7),
				(0,8),  (1,8),  (2,8),  (3,8),
				(0,9),  (1,9),  (2,9),  (3,9),  (14,9),
				(0,10), (1,10), (2,10), (3,10),
				(0,11), (1,11), (2,11), (3,11),
				(0,12), (1,12), (2,12), (3,12), (11,12),
				(0,13), (1,13), (2,13), (3,13),
			].iter().map(|c| Coordinate(c.0, c.1)).collect::<HashSet<Coordinate>>(),
		);
	}
}
