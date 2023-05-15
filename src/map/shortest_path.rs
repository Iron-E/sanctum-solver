use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Build, Coordinate, Tile,
	},
	serde::{Deserialize, Serialize},
	std::collections::{HashMap, HashSet, LinkedList},
};

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ShortestPath(Vec<Coordinate>);

impl ShortestPath {
	/// # Summary
	///
	/// Returns the shorter [`ShortestPath`].
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
	/// Find the shortest [`ShortestPath`] from some `coordiantes` on a `tileset` to any [`Tile`] `needle`
	/// of `needle`'s type.
	///
	/// # Returns
	///
	/// * `Some(ShortestPath)` if there is a [`ShortestPath`].
	/// * `None` if there is no [`ShortestPath`].
	pub fn from_any_to<'coord>(
		grid: &[impl AsRef<[Tile]>],
		build: Option<&Build>,
		start_points: impl Iterator<Item = &'coord Coordinate>,
		end_points: &HashSet<Coordinate>,
	) -> Option<Self> {
		start_points
			.map(|coord| ShortestPath::from_coordinate_to(&grid, build, *coord, &end_points))
			.flatten()
			.reduce(ShortestPath::return_shorter)
	}

	/// # Summary
	///
	/// Get the shortest [`ShortestPath`] to a [`Tile`] of `needle`'s type from some `start`ing [`Coordinate`] on a `tileset`.
	pub fn from_coordinate_to(
		grid: &[impl AsRef<[Tile]>],
		build: Option<&Build>,
		start: Coordinate,
		end_points: &HashSet<Coordinate>,
	) -> Option<Self> {
		let start_tile = start.get_from(&grid).expect(COORDINATE_ON_TILESET);

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

			let tile: Tile = coord
				.get_from_build(&grid, build)
				.expect(COORDINATE_ON_TILESET);

			// Using BFS, so if the `tile` is the `needle` we've found the shortest path.
			if end_points.contains(&coord) {
				return Some(ShortestPath(current_path));
			}
			// Only keep looking beyond a passable tile, and if the current tile is not what we're
			// searching for.
			else if tile.is_passable() {
				Adjacent::<Coordinate>::from_build_coordinate(&grid, build, &coord).for_each(
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
}

impl From<ShortestPath> for Vec<Coordinate> {
	fn from(other: ShortestPath) -> Self {
		other.0
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, ShortestPath, Tileset, COORDINATE_ON_TILESET},
		crate::map::tileset::tests::{PARK, PARK_TWO_SPAWN},
		std::time::Instant,
	};

	fn assertion(tileset: &Tileset, paths: &[ShortestPath], index: usize, desired_len: usize) {
		// Since there may be multiple ways to do this we aren't going to test it
		// directly, rather we're going to assert things about the path instead.
		assert_eq!(paths[index].0.len(), desired_len);
		assert!(paths[index].0.iter().all(|coord| coord
			.get_from(&tileset.grid)
			.expect(COORDINATE_ON_TILESET)
			.is_passable()));
	}

	#[test]
	fn from_any_to() {
		let test_tileset = Tileset::new(
			PARK_TWO_SPAWN
				.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_paths: Vec<_> = test_tileset
			.entrances
			.iter()
			.map(|entrances| {
				ShortestPath::from_any_to(
					&test_tileset.grid,
					None,
					entrances.iter(),
					&test_tileset.exits,
				)
			})
			.flatten()
			.collect();
		println!(
			"ShortestPath::from_any_to {}us",
			Instant::now().duration_since(start).as_micros()
				/ (test_tileset.entrances.len() as u128)
		);

		// There should be two paths to the core since there are two spawn points.
		assert_eq!(test_paths.len(), 2);

		// The shortest path from the left-hand Spawn should be of length nine.
		assertion(&test_tileset, &test_paths, 0, 5);

		// The shortest path from the right-hand Spawn should be of length 15.
		assertion(&test_tileset, &test_paths, 1, 7);
	}

	#[test]
	fn from_coordinate_to() {
		let test_tileset = Tileset::new(
			PARK.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_path = ShortestPath::from_coordinate_to(
			&test_tileset.grid,
			None,
			Coordinate(4, 4),
			&test_tileset.exits,
		)
		.unwrap();
		println!(
			"ShortestPath::from_coordinate_to {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assertion(&test_tileset, &[test_path], 0, 6);
	}
}
