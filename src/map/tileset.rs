mod error;

use std::collections::{HashMap, HashSet, LinkedList};

pub use error::{Error, Result};
use serde::{Deserialize, Serialize};

use super::{Adjacent, Coordinate, Tile};

pub const COORDINATE_ON_TILESET: &str = "Expected to visit coordinate which exists on tileset.";
const IS_REGION: &str = "Expected to separate tiles which are regions.";
pub const REGION_HAS_COORDINATE: &str = "Expected the region to have at least one coordinate.";

/// # Summary
///
/// T two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tileset
{
	pub grid: Vec<Vec<Tile>>,
	pub entrances_by_region: Vec<HashMap<Coordinate, usize>>,
}

impl Tileset
{
	/// # Summary
	///
	/// Select all of the [`Tile::Empty`]s next to [`Tile::Spawn`] points on this [`Tileset`].
	fn entrances(tileset: &[impl AsRef<[Tile]>]) -> Vec<HashMap<Coordinate, usize>>
	{
		Self::separate_regions(tileset, Tile::Spawn)
			.expect(IS_REGION)
			.into_iter()
			.map(|region| {
				// get a random point on the region and look for adjacent empty tiles
				Self::get_adjacent_to(
					tileset,
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
	pub fn get_adjacent_to(
		grid: &[impl AsRef<[Tile]>],
		start: Coordinate,
		needle: Tile,
	) -> HashMap<Coordinate, usize>
	{
		let start_tile = start.get_from(&grid).expect(COORDINATE_ON_TILESET);

		let mut coordinate_queue = LinkedList::new();
		let mut visited = HashMap::new();

		coordinate_queue.push_back(start);

		while let Some(coord) = coordinate_queue.pop_front()
		{
			// Don't revisit a coordinate we've already been to.
			if visited.contains_key(&coord)
			{
				continue;
			}

			// All of the coordinates from `select` should exist in the `tileset`.
			let tile = coord.get_from(&grid).expect(COORDINATE_ON_TILESET);

			// We shouldn't count a coordinate as 'visited' until we can extract its tile value.
			visited.insert(coord, tile);

			// These are the tiles which we want to keep looking beyond.
			if (start_tile.is_region() && tile == start_tile) ||
				(tile.is_passable() && tile != needle)
			{
				Adjacent::from_grid_coordinate(&grid, &coord, false)
					.for_each(|adjacent_coord| coordinate_queue.push_back(adjacent_coord));
			}
		}

		// Whatever we visited which was an `Empty` tile, return.
		visited
			.into_iter()
			.filter(|(_, tile)| tile == &needle)
			.map(|(coord, _)| (coord, coord.distance_from(&start)))
			.collect()
	}

	/// # Summary
	///
	/// Create a new [`Tileset`] from some two-dimensional `grid` of [`Tile`]s.
	pub fn new(grid: Vec<Vec<Tile>>) -> Self
	{
		Self { entrances_by_region: Self::entrances(&grid), grid }
	}

	/// # Summary
	///
	/// Get all of the different regions for some type of `tile`.
	fn separate_regions(
		tileset: &[impl AsRef<[Tile]>],
		start_tile: Tile,
	) -> Result<Vec<HashSet<Coordinate>>>
	{
		if !start_tile.is_region()
		{
			return Err(Error::NotRegion { tile: start_tile });
		}

		let mut buckets = Vec::<HashSet<Coordinate>>::new();

		let get_region = |start: Coordinate| -> HashSet<Coordinate> {
			let mut coordinate_queue = LinkedList::new();
			let mut visited = HashSet::new();

			coordinate_queue.push_back(start);

			while let Some(coord) = coordinate_queue.pop_front()
			{
				// Don't revisit a coordinate we've already been to.
				if visited.contains(&coord)
				{
					continue;
				}

				// All of the coordinates from `select` should exist in the `tileset`.
				let tile = coord.get_from(&tileset).expect(COORDINATE_ON_TILESET);

				// These are the tiles which we want to keep looking beyond.
				if tile == start_tile
				{
					// We shouldn't count a coordinate as 'visited' until we can extract its tile
					// value.
					visited.insert(coord);

					Adjacent::from_grid_coordinate(&tileset, &coord, false)
						.for_each(|adjacent_coord| coordinate_queue.push_back(adjacent_coord));
				}
			}

			visited
		};

		tileset.iter().enumerate().for_each(|(y, row)| {
			row.as_ref()
				.iter()
				.enumerate()
				.filter(|(_, row_value)| *row_value == &start_tile)
				.for_each(|(x, _)| {
					let coord = Coordinate(x, y);
					if buckets.iter().all(|set| !set.contains(&coord))
					{
						buckets.push(get_region(Coordinate(x, y)))
					}
				})
		});

		Ok(buckets)
	}
}

#[cfg(test)]
pub mod tests
{
	use std::time::Instant;

	use super::{Coordinate, Tile, Tile::*, Tileset};

	/// # Summary
	///
	/// T representation of the map _Park_ from _Sanctum 2_.
	#[rustfmt::skip]
	pub const PARK: [[Tile; 16]; 14] = [
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

	/// # Summary
	///
	/// T representation of the map _Park_ from _Sanctum 2_.
	#[rustfmt::skip]
	pub const PARK_TWO_SPAWN: [[Tile; 16]; 14] = [
		// 0     1       2       3       4       5       6       7       8       9       10      11      12     13     14      15
		[Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Impass, Empty,  Empty, Empty, Empty,  Empty], // 0
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Impass, Impass, Empty,  Empty, Empty, Empty,  Empty], // 1
		[Spawn,  Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Impass, Impass, Empty,  Empty, Empty, Empty,  Empty], // 2
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 3
		[Pass,   Pass,   Pass,   Pass,   Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Empty], // 4
		[Impass, Impass, Impass, Impass, Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty, Empty, Empty,  Spawn], // 5
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
	fn entrances()
	{
		let start = Instant::now();
		let entrances = Tileset::entrances(&PARK);
		println!("Tileset::entrances {}us", Instant::now().duration_since(start).as_micros());

		assert_eq!(entrances.len(), 1);
		assert_eq!(
			entrances.first().unwrap(),
			&[
				(Coordinate(4, 1), 5),
				(Coordinate(4, 2), 4),
				(Coordinate(4, 3), 5),
				(Coordinate(4, 4), 6),
			]
			.iter()
			.copied()
			.collect()
		)
	}

	#[test]
	fn separate_regions()
	{
		let start = Instant::now();
		let core_regions = Tileset::separate_regions(&PARK_TWO_SPAWN, Tile::Core).unwrap();
		let spawn_regions = Tileset::separate_regions(&PARK_TWO_SPAWN, Tile::Spawn).unwrap();
		println!(
			"Tileset::separate_regions {}us",
			Instant::now().duration_since(start).as_micros() / 2
		);

		// Only one `core` region
		assert_eq!(core_regions.len(), 1);
		assert_eq!(
			core_regions[0],
			[Coordinate(5, 11), Coordinate(5, 12), Coordinate(6, 11), Coordinate(6, 12)]
				.iter()
				.copied()
				.collect()
		);

		// Can't get a non-region.
		assert!(Tileset::separate_regions(&PARK_TWO_SPAWN, Tile::Impass).is_err());

		// Two `spawn` regions
		assert_eq!(spawn_regions.len(), 2);
		assert_eq!(spawn_regions[0], [Coordinate(0, 2)].iter().copied().collect());
		assert_eq!(spawn_regions[1], [Coordinate(15, 5)].iter().copied().collect());
	}
}
