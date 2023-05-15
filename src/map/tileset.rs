mod error;

pub use error::{Error, Result};

use {
	super::{Adjacent, Coordinate, Tile},
	serde::{Deserialize, Serialize},
	std::collections::{HashMap, HashSet},
};

const COORDINATE_ON_TILESET: &str = "Expected to visit coordinate which exists on tileset.";
const IS_REGION: &str = "Expected to separate tiles which are regions.";
const REGION_HAS_COORDINATE: &str = "Expected the region to have at least one coordinate.";

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tileset(pub Vec<Vec<Tile>>);

impl Tileset {
	/// # Summary
	///
	/// Select all of the [`Tile::Empty`]s next to [`Tile::Spawn`] points on this [`Tileset`].
	pub fn entrances(&self) -> Vec<HashSet<Coordinate>> {
		self.separate_regions(Tile::Spawn)
			.expect(IS_REGION)
			.into_iter()
			.map(|region| {
				// get a random point on the region and look for adjacent empty tiles
				self.get_adjacent_to(
					region.into_iter().next().expect(REGION_HAS_COORDINATE),
					Tile::Empty,
				)
			})
			.collect()
	}

	/// # Summary
	///
	/// Select all of the [`Tile::Empty`]s next to [`Tile::Core`] points on this [`Tileset`].
	pub fn exits(&self) -> Vec<HashSet<Coordinate>> {
		self.separate_regions(Tile::Core)
			.expect(IS_REGION)
			.into_iter()
			.map(|region| {
				// get a random point on the region and look for adjacent empty tiles
				self.get_adjacent_to(
					region.into_iter().next().expect(REGION_HAS_COORDINATE),
					Tile::Empty,
				)
			})
			.collect()
	}

	/// # Summary
	///
	/// Get the adjacent [`Tile`]s of `needle`'s type which are adjecent to the `start`ing
	/// [`Coordinate`].
	fn get_adjacent_to(&self, start: Coordinate, needle: Tile) -> HashSet<Coordinate> {
		let start_tile = start.get_from(&self.0).expect(COORDINATE_ON_TILESET);

		let mut coordinate_queue = vec![start];
		let mut visited = HashSet::new();

		while let Some(coord) = coordinate_queue.pop() {
			// Don't revisit a coordinate we've already been to.
			if visited.contains(&coord) {
				continue;
			}

			// All of the coordinates from `select` should exist in the `tileset`.
			let tile = coord.get_from(&self.0).expect(COORDINATE_ON_TILESET);

			// We shouldn't count a coordinate as 'visited' until we can extract its tile value.
			visited.insert(coord);

			// These are the tiles which we want to keep looking beyond.
			if (start_tile.is_region() && tile == start_tile)
				|| (tile.is_passable() && tile != needle)
			{
				Adjacent::<Coordinate>::from_array_coordinate(&self.0, &coord)
					.for_each(|adjacent| coordinate_queue.push(adjacent));
			}
		}

		// Whatever we visited which was an `Empty` tile, return.
		visited
			.into_iter()
			.filter(|coord| coord.get_from(&self.0).expect(COORDINATE_ON_TILESET) == needle)
			.collect()
	}

	/// # Summary
	///
	/// Get the [`Tile`] of `needle`'s type which is nearest to the `start`ing [`Coordinate`].
	///
	/// # Remarks
	///
	/// If multiple candidates are found to be of the same distance, they will all be returned.
	fn get_nearest(&self, start: Coordinate, needle: Tile) -> Result<HashSet<Coordinate>> {
		let start_tile = start.get_from(&self.0).expect(COORDINATE_ON_TILESET);

		// We don't want to start the search on a tile which cannot be walked over.
		// This is to prevent accidentally crossing over the other side of a barrier.
		if !start_tile.is_passable() {
			return Err(Error::CannotPass { tile: start_tile });
		}

		let mut coordinate_distance_queue = vec![(start, 0)];
		let mut visited = HashMap::new();

		while let Some((coord, distance)) = coordinate_distance_queue.pop() {
			// Don't revisit a coordinate we've already been to.
			if match visited.get(&coord) {
				Some(d) => d > &distance,
				_ => false,
			} {
				continue;
			}

			// All of the coord_distanceinates from `select` should exist in the `tileset`.
			let tile = coord.get_from(&self.0).expect(COORDINATE_ON_TILESET);

			// We shouldn't count a coord_distanceinate as 'visited' until we can extract its tile value.
			visited.insert(coord, distance);

			// These are the tiles which we want to keep looking beyond.
			if tile.is_passable() && tile != needle {
				Adjacent::<Coordinate>::from_array_coordinate(&self.0, &coord)
					.for_each(|adjacent| coordinate_distance_queue.push((adjacent, distance + 1)));
			}
		}

		let shortest_distance = match visited
			.iter()
			.filter(|(c, _)| c.get_from(&self.0).expect(COORDINATE_ON_TILESET) == needle)
			.reduce(|visit, other_visit| {
				if visit.1 > other_visit.1 {
					other_visit
				} else {
					visit
				}
			}) {
			Some(visit) => *visit.1,
			_ => return Ok(HashSet::new()),
		};

		Ok(visited
			.into_iter()
			.filter_map(|(c, d)| {
				if d == shortest_distance
					&& c.get_from(&self.0).expect(COORDINATE_ON_TILESET) == needle
				{
					Some(c)
				} else {
					None
				}
			})
			.collect())
	}

	/// # Summary
	///
	/// Get all of the different regions for some type of `tile`.
	fn separate_regions(&self, start_tile: Tile) -> Result<Vec<HashSet<Coordinate>>> {
		if !start_tile.is_region() {
			return Err(Error::NotRegion { tile: start_tile });
		}

		let mut buckets = Vec::<HashSet<Coordinate>>::new();

		let get_region = |start: Coordinate| -> HashSet<Coordinate> {
			let mut coordinate_queue = vec![start];
			let mut visited = HashSet::new();

			while let Some(coord) = coordinate_queue.pop() {
				// Don't revisit a coordinate we've already been to.
				if visited.contains(&coord) {
					continue;
				}

				// All of the coordinates from `select` should exist in the `tileset`.
				let tile = coord.get_from(&self.0).expect(COORDINATE_ON_TILESET);

				// These are the tiles which we want to keep looking beyond.
				if tile == start_tile {
					// We shouldn't count a coordinate as 'visited' until we can extract its tile value.
					visited.insert(coord);

					Adjacent::<Coordinate>::from_array_coordinate(&self.0, &coord)
						.for_each(|adjacent| coordinate_queue.push(adjacent));
				}
			}

			visited
		};

		self.0.iter().enumerate().for_each(|(y, row)| {
			row.iter()
				.enumerate()
				.filter(|(_, row_value)| *row_value == &start_tile)
				.for_each(|(x, _)| {
					let coord = Coordinate(x, y);
					if buckets.iter().all(|set| !set.contains(&coord)) {
						buckets.push(get_region(Coordinate(x, y)))
					}
				})
		});

		Ok(buckets)
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, Tile, Tile::*, Tileset},
		std::time::Instant,
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

		assert_eq!(entrances.len(), 1);
		assert_eq!(
			entrances.first().unwrap(),
			&[
				Coordinate(4, 1),
				Coordinate(4, 2),
				Coordinate(4, 3),
				Coordinate(4, 4),
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

		assert_eq!(exits.len(), 1);
		assert_eq!(
			exits.first().unwrap(),
			&[
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
	fn separate_regions() {
		#[rustfmt::skip]
		let test = Tileset(vec![
			//   0       1       2       3       4
			vec![Impass, Impass, Impass, Impass, Impass], // 0
			vec![Impass, Empty,  Empty,  Spawn,  Impass], // 1
			vec![Impass, Empty,  Empty,  Spawn,  Impass], // 2
			vec![Impass, Empty,  Empty,  Empty,  Impass], // 3
			vec![Impass, Empty,  Empty,  Empty,  Impass], // 4
			vec![Impass, Spawn,  Empty,  Core,   Impass], // 5
			vec![Impass, Spawn,  Empty,  Core,   Impass], // 6
			vec![Impass, Impass, Impass, Impass, Impass], // 7
		]);

		let start = Instant::now();
		let core_regions = test.separate_regions(Tile::Core).unwrap();
		let impass_regions = test.separate_regions(Tile::Impass);
		let spawn_regions = test.separate_regions(Tile::Spawn).unwrap();
		println!(
			"Tileset::separate_regions {}us",
			Instant::now().duration_since(start).as_micros()
		);

		// Only one `core` region
		assert_eq!(core_regions.len(), 1);
		assert_eq!(
			core_regions[0],
			[Coordinate(3, 5), Coordinate(3, 6)]
				.iter()
				.copied()
				.collect()
		);

		// Can't get a non-region.
		assert!(impass_regions.is_err());

		// Two `spawn` regions
		assert_eq!(spawn_regions.len(), 2);
		assert_eq!(
			spawn_regions[0],
			[Coordinate(3, 1), Coordinate(3, 2)]
				.iter()
				.copied()
				.collect()
		);
		assert_eq!(
			spawn_regions[1],
			[Coordinate(1, 5), Coordinate(1, 6)]
				.iter()
				.copied()
				.collect()
		);
	}
}
