#![allow(clippy::len_without_is_empty)]

use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Coordinate, Tile,
	},
	crate::Container,
	rayon::iter::{IntoParallelRefIterator, ParallelIterator},
	serde::{Deserialize, Serialize},
	std::{
		cmp::Ordering,
		collections::{HashMap, LinkedList},
	},
};

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ShortestPath(Vec<Coordinate>);

impl ShortestPath {
	/// # Summary
	///
	/// Return the [`Tile::Core`] which this [`ShortestPath`] navigates to.
	pub fn core(&self) -> Coordinate {
		*self
			.0
			.last()
			.expect("Expected this `ShortestPath` to have at least 1 coordinate")
	}

	/// # Summary
	///
	/// Find the shortest [`ShortestPath`] from some `start_points` on a `grid` to any [`Tile`]
	/// of `end_tile`'s type.
	///
	/// # Returns
	///
	/// * `Some(ShortestPath)` if there is a [`ShortestPath`].
	/// * `None` if there is no [`ShortestPath`].
	pub fn from_any_grid_coordinate_to_tile<'coord>(
		grid: &[impl AsRef<[Tile]> + Send + Sync],
		build: Option<&impl Container<Coordinate>>,
		start_points: impl ParallelIterator<Item = &'coord Coordinate>,
		end_tile: Tile,
	) -> Option<Self> {
		start_points
			.map(|coord| ShortestPath::from_grid_coordinate_to_tile(&grid, build, *coord, end_tile))
			.flatten()
			.reduce_with(ShortestPath::return_shorter)
	}

	/// # Summary
	///
	/// Get the [`ShortestPath`]s from all [`Tileset::entrances`] to any [`Tileset::exits`].
	pub fn from_entrances_to_any_core(
		tileset: &Tileset,
		build: Option<&impl Container<Coordinate>>,
	) -> Vec<Option<Self>> {
		tileset
			.entrances_by_region
			.par_iter()
			.map(|entrances| {
				ShortestPath::from_any_grid_coordinate_to_tile(
					&tileset.grid,
					build,
					entrances.par_iter(),
					Tile::Core,
				)
			})
			.collect()
	}

	/// # Summary
	///
	/// Get the shortest [`ShortestPath`] to a [`Tile`] of `end_tile`'s type from some `start`ing [`Coordinate`] on a `tileset`.
	pub fn from_grid_coordinate_to_tile(
		grid: &[impl AsRef<[Tile]>],
		build: Option<&impl Container<Coordinate>>,
		start: Coordinate,
		end_point: Tile,
	) -> Option<Self> {
		let start_tile = start
			.get_from_with_build(&grid, build)
			.expect(COORDINATE_ON_TILESET);

		// We don't want to start the search on a tile which cannot be walked over.
		// This is to prevent accidentally crossing over the other side of a barrier.
		if !start_tile.is_passable() {
			return None;
		}

		let mut coordinate_path_queue = LinkedList::new();
		let mut visited = HashMap::new();

		coordinate_path_queue.push_back((start, vec![start]));

		while let Some((coord, current_path)) = coordinate_path_queue.pop_front() {
			// If the current path is longer than the previous path (defaulting to `false` if there
			// is no previous path).
			if match visited.get(&coord) {
				Some(visited_path_len) => &current_path.len() >= visited_path_len,
				_ => false,
			} {
				continue;
			}

			let tile: Tile = coord
				.get_from_with_build(&grid, build)
				.expect(COORDINATE_ON_TILESET);

			// Using BFS, so if the `tile` is the `end_tile` we've found the shortest path.
			if tile == end_point {
				return Some(ShortestPath(current_path));
			}
			// Only keep looking beyond a passable tile, and if the current tile is not what we're
			// searching for.
			else if tile.is_passable() {
				Adjacent::from_grid_coordinate_with_build(&grid, build, &coord).for_each(
					|adjacent_coord| {
						let mut new_path = Vec::with_capacity(current_path.len() + 1);
						new_path.extend_from_slice(&current_path);
						new_path.push(adjacent_coord);

						coordinate_path_queue.push_back((adjacent_coord, new_path))
					},
				);
			}

			// Now that the current coordinate has been fully evaluated, mark it as visited.
			visited.insert(coord, current_path.len());
		}

		None
	}

	/// # Summary
	///
	/// The length of the path.
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// # Summary
	///
	/// Returns the shorter [`ShortestPath`].
	///
	/// # Remarks
	///
	/// If paths are equally long, the current path is preferred.
	fn return_shorter(self, other: Self) -> Self {
		if self.len() > other.len() {
			return other;
		}
		self
	}
}

impl From<ShortestPath> for Vec<Coordinate> {
	fn from(other: ShortestPath) -> Self {
		other.0
	}
}

impl Ord for ShortestPath {
	fn cmp(&self, other: &Self) -> Ordering {
		self.len().cmp(&other.len())
	}
}

impl PartialOrd for ShortestPath {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.len().partial_cmp(&other.len())
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, ShortestPath, Tile, Tileset, COORDINATE_ON_TILESET},
		crate::map::tileset::tests::{PARK, PARK_TWO_SPAWN},
		rayon::iter::IntoParallelRefIterator,
		std::{collections::HashSet, time::Instant},
	};

	fn assertion(tileset: &Tileset, paths: &[ShortestPath], index: usize, desired_len: usize) {
		// Since there may be multiple ways to do this we aren't going to test it
		// directly, rather we're going to assert things about the path instead.
		assert_eq!(paths[index].len(), desired_len);
		assert!(paths[index].0[0..(desired_len - 1)]
			.iter()
			.all(|coord| coord
				.get_from(&tileset.grid)
				.expect(COORDINATE_ON_TILESET)
				.is_passable()));
		assert!(paths[index].0[desired_len - 1]
			.get_from(&tileset.grid)
			.expect(COORDINATE_ON_TILESET)
			.is_region());
	}

	#[test]
	fn from_any_grid_coordinate_to_tile() {
		let test_tileset = Tileset::new(
			PARK_TWO_SPAWN
				.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_paths: Vec<_> = test_tileset
			.entrances_by_region
			.iter()
			.map(|entrances| {
				ShortestPath::from_any_grid_coordinate_to_tile(
					&test_tileset.grid,
					Option::<&HashSet<_>>::None,
					entrances.par_iter(),
					Tile::Core,
				)
			})
			.flatten()
			.collect();
		println!(
			"ShortestPath::from_any_grid_coordinate_to_tile {}us",
			Instant::now().duration_since(start).as_micros()
				/ (test_tileset.entrances_by_region.len() as u128)
		);

		let start = Instant::now();
		let test_from_entrances_to_any_core =
			ShortestPath::from_entrances_to_any_core(&test_tileset, Option::<&HashSet<_>>::None)
				.into_iter()
				.flatten()
				.collect::<Vec<_>>();
		println!(
			"ShortestPath::from_entrances_to_any_core {}us",
			Instant::now().duration_since(start).as_micros()
		);

		// The above should be equal to `from_entrances_to_any_exit`.
		assert_eq!(test_paths, test_from_entrances_to_any_core,);

		// There should be two paths to the core since there are two spawn points.
		assert_eq!(test_paths.len(), 2);

		// The shortest path from the left-hand Spawn should be of length nine.
		assertion(&test_tileset, &test_paths, 0, 7);

		// The shortest path from the right-hand Spawn should be of length 15.
		assertion(&test_tileset, &test_paths, 1, 9);
	}

	#[test]
	fn from_grid_coordinate_to_tile() {
		let test_tileset = Tileset::new(
			PARK.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_path = ShortestPath::from_grid_coordinate_to_tile(
			&test_tileset.grid,
			Option::<&HashSet<_>>::None,
			Coordinate(4, 4),
			Tile::Core,
		)
		.unwrap();
		println!(
			"ShortestPath::from_grid_coordinate_to_tile {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assertion(&test_tileset, &[test_path], 0, 8);
	}
}
