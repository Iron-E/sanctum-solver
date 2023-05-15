use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Coordinate, Tile,
	},
	serde::{Deserialize, Serialize},
	std::collections::{HashMap, HashSet, LinkedList},
};

pub const PATH_EXISTS: &str = "Expected path to have at least one coordinate.";

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Path(Vec<Coordinate>);

impl Path {
	/// # Summary
	///
	/// Return the [`Coordinate`]s which this [`Path`] contains.
	pub fn coordinates(self) -> Vec<Coordinate> {
		self.0
	}

	/// # Summary
	///
	/// Returns the shorter [`Path`].
	///
	/// # Remarks
	///
	/// If paths are equally long, the current path is preferred.
	fn return_shorter(self, other: Self) -> Self {
		if self.0.len() > other.0.len() {
			return other;
		}

		self
	}

	/// # Summary
	///
	/// Find the shortest [`Path`] from some `coordiantes` on a `tileset` to any [`Tile`] `needle`
	/// of `needle`'s type.
	///
	/// # Returns
	///
	/// * `Some(Path)` if there is a [`Path`].
	/// * `None` if there is no [`Path`].
	pub fn shortest_from_any_to<'coord>(
		tileset: &Tileset<impl AsRef<[Tile]>>,
		start_points: impl Iterator<Item = &'coord Coordinate>,
		end_points: &HashSet<Coordinate>,
	) -> Option<Self>
	{
		start_points
			.map(|coord| Path::shortest_from_coordinate_to(tileset, *coord, &end_points))
			.flatten()
			.reduce(Path::return_shorter)
	}

	/// # Summary
	///
	/// Get the shortest [`Path`] to a [`Tile`] of `needle`'s type from some `start`ing [`Coordinate`] on a `tileset`.
	pub fn shortest_from_coordinate_to(
		tileset: &Tileset<impl AsRef<[Tile]>>,
		start: Coordinate,
		end_points: &HashSet<Coordinate>,
	) -> Option<Self>
	{
		let start_tile = start.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET);

		// We don't want to start the search on a tile which cannot be walked over.
		// This is to prevent accidentally crossing over the other side of a barrier.
		if !start_tile.is_passable() {
			return None;
		}

		let mut coordinate_path_queue = LinkedList::new();
		let mut visited = HashMap::<Coordinate, Vec<Coordinate>>::new();

		coordinate_path_queue.push_back((start, vec![start]));

		while let Some((coord, current_path)) = coordinate_path_queue.pop_front() {
			// If the current path is longer than the previous path (defaulting to `false` if there
			// is no previous path).
			if match visited.get(&coord) {
				Some(visited_path) => current_path.len() >= visited_path.len(),
				_ => false,
			} {
				continue;
			}

			let tile = coord.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET);

			// Using BFS, so if the `tile` is the `needle` we've found the shortest path.
			if end_points.contains(&coord) {
				return Some(Path(current_path));
			}
			// Only keep looking beyond a passable tile, and if the current tile is not what we're
			// searching for.
			else if tile.is_passable() {
				Adjacent::<Coordinate>::from_array_coordinate(&tileset.grid, &coord).for_each(
					|adjacent| {
						let mut new_path = Vec::with_capacity(current_path.len() + 1);
						new_path.extend_from_slice(&current_path);
						new_path.push(adjacent);

						coordinate_path_queue.push_back((adjacent, new_path))
					},
				);
			}

			// Now that the current coordinate has been fully evaluated, mark it as visited.
			visited.insert(coord, current_path);
		}

		None
	}

	/// # Summary
	///
	/// Get the [`Path`]s from all [`Tileset::entrances`] to any [`Tileset::exits`].
	pub fn shortest_from_entrances_to_any_exit(tileset: &Tileset<impl AsRef<[Tile]>>) -> Vec<Option<Self>>
	{
		tileset
			.entrances
			.iter()
			.map(|entrances| Path::shortest_from_any_to(&tileset, entrances.iter(), &tileset.exits))
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, Path, Tile, Tileset, COORDINATE_ON_TILESET},
		crate::map::tileset::tests::{PARK, PARK_TWO_SPAWN},
		std::time::Instant,
	};

	fn assertion(tileset: &Tileset<impl AsRef<[Tile]>>, paths: &[Path], index: usize, desired_len: usize) {
		// Since there may be multiple ways to do this we aren't going to test it
		// directly, rather we're going to assert things about the path instead.
		assert_eq!(paths[index].0.len(), desired_len);
		assert!(paths[index].0
			.iter()
			.all(|coord| coord
				.get_from(&tileset.grid)
				.expect(COORDINATE_ON_TILESET)
				.is_passable()));
	}

	#[test]
	fn shortest_from_any_to() {
		let test_tileset = Tileset::new(PARK_TWO_SPAWN.iter().collect());

		let start = Instant::now();
		let test_paths: Vec<_> = test_tileset
			.entrances
			.iter()
			.map(|entrances| Path::shortest_from_any_to(&test_tileset, entrances.iter(), &test_tileset.exits))
			.flatten()
			.collect();
		println!(
			"Path::shortest_from_any_to {}us",
			Instant::now().duration_since(start).as_micros()
				/ (test_tileset.entrances.len() as u128)
		);

		// There should be two paths to the core since there are two spawn points.
		assert_eq!(test_paths.len(), 2);

		// The shortest path from the left-hand Spawn should be of length nine.
		assertion(&test_tileset, &test_paths, 0, 6);

		// The shortest path from the right-hand Spawn should be of length 15.
		assertion(&test_tileset, &test_paths, 1, 12);
	}


	#[test]
	fn shortest_from_coordinate_to() {
		let test_tileset = Tileset::new(PARK.iter().collect());

		let start = Instant::now();
		let test_path =
			Path::shortest_from_coordinate_to(&test_tileset, Coordinate(4, 4), &test_tileset.exits)
				.unwrap();
		println!(
			"Path::shortest_from_coordinate_to {}us",
			Instant::now().duration_since(start).as_micros()
		);

		let desired_len = 6;
		assert_eq!(test_path.0.len(), desired_len);
		assert!(test_path.0[..(desired_len - 1)].into_iter().all(|coord| coord
			.get_from(&test_tileset.grid)
			.expect(COORDINATE_ON_TILESET)
			.is_passable()));
	}

	#[test]
	fn shortest_from_entrances_to_any_exit() {
		let test_tileset = Tileset::new(PARK_TWO_SPAWN.iter().collect());

		let start = Instant::now();
		let test_paths: Vec<_> = Path::shortest_from_entrances_to_any_exit(&test_tileset).into_iter().flatten().collect();
		println!(
			"Path::shortest_from_entrances_to_any_exit {}us",
			Instant::now().duration_since(start).as_micros()
		);

		// There should be two paths to the core since there are two spawn points.
		assert_eq!(test_paths.len(), 2);

		// The shortest path from the left-hand Spawn should be of length nine.
		assertion(&test_tileset, &test_paths, 0, 6);

		// The shortest path from the right-hand Spawn should be of length 15.
		assertion(&test_tileset, &test_paths, 1, 12);
	}
}
