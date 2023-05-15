use {
	super::{Adjacent, Coordinate, Tile},
	serde::{Deserialize, Serialize},
	std::collections::{HashMap, HashSet},
};

const COORDINATE_NOT_ON_TILESET: &str = "Tried to visit non-existing coordiante";

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tileset(pub Vec<Vec<Tile>>);

impl Tileset {
	/// # Summary
	///
	/// Select all of the [`Tile::Empty`]s next to [`Tile::Spawn`] points on this [`Tileset`].
	pub fn entrances(&self) -> HashSet<Coordinate> {
		self.get_all_adjacent_to(Tile::Empty, Tile::Spawn)
	}

	/// # Summary
	///
	/// Select all of the [`Tile::Empty`]s next to [`Tile::Core`] points on this [`Tileset`].
	pub fn exits(&self) -> HashSet<Coordinate> {
		self.get_all_adjacent_to(Tile::Empty, Tile::Core)
	}

	/// # Summary
	///
	/// Return a [set](HashSet) of every specific [`Tile`] in the [`Tileset`].
	///
	/// # Remarks
	///
	/// Iterates over each row and row-value of the [`Self::tileset`], filtering out those which are
	/// not the same value as `tile`.
	pub fn get_all(&self, tile: &Tile) -> HashSet<Coordinate> {
		self.0
			.iter()
			.enumerate()
			.flat_map(|(y, row)| {
				row.iter().enumerate().filter_map(move |(x, value)| {
					if value == tile {
						Some(Coordinate(x, y))
					} else {
						None
					}
				})
			})
			.collect()
	}

	/// # Summary
	///
	/// Get all `end_tile`s adjacent to `start_tile`.
	fn get_all_adjacent_to(&self, end_tile: Tile, start_tile: Tile) -> HashSet<Coordinate> {
		let mut coordinate_queue: Vec<Coordinate> = self.get_all(&start_tile).into_iter().collect();
		let mut visited = HashSet::new();

		while let Some(coord) = coordinate_queue.pop() {
			// Don't revisit a coordinate we've already been to.
			if visited.contains(&coord) {
				continue;
			}

			// All of the coordinates from `select` should exist in the `tileset`.
			let tile = coord.get_from(&self.0).expect(COORDINATE_NOT_ON_TILESET);

			// We shouldn't count a coordinate as 'visited' until we can extract its tile value.
			visited.insert(coord);

			// These are the tiles which we want to keep looking beyond.
			if (start_tile.is_region() && tile == start_tile)
				|| (tile.is_passable() && tile != end_tile)
			{
				Adjacent::<Coordinate>::from_array_coordinate(&self.0, &coord)
					.into_iter()
					.flatten()
					.for_each(|coord| coordinate_queue.push(coord));
			}
		}

		// Whatever we visited which was an `Empty` tile, return.
		visited
			.into_iter()
			.filter(|coord| coord.get_from(&self.0).expect(COORDINATE_NOT_ON_TILESET) == end_tile)
			.collect()
	}

	/// # Summary
	///
	/// Get all `end_tile`s nearest to `start_tile`.
	fn get_all_nearest_to(&self, end_tile: Tile, start: Coordinate) -> HashSet<Coordinate> {
		let mut coordinate_queue: Vec<Coordinate> = vec![start];
		let mut visited = HashSet::new();

		let start_tile = start.get_from(&self.0).expect(COORDINATE_NOT_ON_TILESET);

		while let Some(coord) = coordinate_queue.pop() {
			// Don't revisit a coordinate we've already been to.
			if visited.contains(&coord) {
				continue;
			}

			// All of the coordinates from `select` should exist in the `tileset`.
			let tile = coord.get_from(&self.0).expect(COORDINATE_NOT_ON_TILESET);

			// We shouldn't count a coordinate as 'visited' until we can extract its tile value.
			visited.insert(coord);

			// These are the tiles which we want to keep looking beyond.
			if (start_tile.is_region() && tile == start_tile)
				|| (tile.is_passable() && tile != end_tile)
			{
				Adjacent::<Coordinate>::from_array_coordinate(&self.0, &coord)
					.into_iter()
					.flatten()
					.for_each(|coord| coordinate_queue.push(coord));
			}
		}

		let mut distances = (usize::MAX, Vec::new());
		for visit in visited
		{
			let distance = start.distance_from(visit);

			if distances.0 > distance
			{
				distances.1 = vec![visit];
			}
			else if distances.0 == distance
			{
				distances.1.push(visit);
			}
		}

		distances.1.into_iter().collect()
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, Tile, Tile::*, Tileset},
		std::{collections::HashSet, time::Instant},
	};

	/// # Summary
	///
	/// A representation of the map _Park_ from _Sanctum 2_.
	#[rustfmt::skip]
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
	fn entrances() {
		let park = Tileset(
			PARK.iter()
				.map(|row| row.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let entrances = park.entrances();
		println!(
			"Tileset::entrances {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(
			entrances,
			[
				Coordinate(4, 1),
				Coordinate(4, 2),
				Coordinate(4, 3),
				Coordinate(4, 4)
			]
			.iter()
			.copied()
			.collect()
		)
	}

	#[test]
	fn exits() {
		let park = Tileset(
			PARK.iter()
				.map(|row| row.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let exits = park.exits();
		println!(
			"Tileset::exits {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(
			exits,
			[
				Coordinate(4, 9),
				Coordinate(5, 9),
				Coordinate(6, 9),
				Coordinate(7, 9),
				Coordinate(8, 10),
				Coordinate(8, 11),
				Coordinate(8, 12),
				Coordinate(8, 13),
			]
			.iter()
			.copied()
			.collect()
		)
	}

	#[test]
	fn get_all() {
		let park = Tileset(
			PARK.iter()
				.map(|row| row.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let impasses = park.get_all(&Tile::Impass);
		println!(
			"Tileset::select {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(
			impasses,
			[
				(0, 0),
				(1, 0),
				(2, 0),
				(3, 0),
				(4, 0),
				(5, 0),
				(6, 0),
				(7, 0),
				(8, 0),
				(9, 0),
				(10, 0),
				(9, 1),
				(10, 1),
				(9, 2),
				(10, 2),
				(0, 5),
				(1, 5),
				(2, 5),
				(3, 5),
				(0, 6),
				(1, 6),
				(2, 6),
				(3, 6),
				(0, 7),
				(1, 7),
				(2, 7),
				(3, 7),
				(0, 8),
				(1, 8),
				(2, 8),
				(3, 8),
				(0, 9),
				(1, 9),
				(2, 9),
				(3, 9),
				(14, 9),
				(0, 10),
				(1, 10),
				(2, 10),
				(3, 10),
				(0, 11),
				(1, 11),
				(2, 11),
				(3, 11),
				(0, 12),
				(1, 12),
				(2, 12),
				(3, 12),
				(11, 12),
				(0, 13),
				(1, 13),
				(2, 13),
				(3, 13),
			]
			.iter()
			.map(|c| Coordinate(c.0, c.1))
			.collect::<HashSet<Coordinate>>(),
		);
	}
}
