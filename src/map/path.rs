use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Coordinate, Tile,
	},
	serde::{Deserialize, Serialize},
	std::collections::{HashMap, HashSet, LinkedList},
};

pub const PATH_HAS_COORDINATE: &str = "Expected path to have at least one coordinate.";

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Path(pub Vec<Coordinate>);

impl Path {
	/// # Summary
	///
	/// Get the shortest [`Path`] to a [`Tile`] of `needle`'s type from some `start`ing [`Coordinate`] on a `tileset`.
	fn from_coordinate(tileset: &Tileset, start: Coordinate, needle: Tile) -> Option<Self> {
		let start_tile = start.get_from(&tileset.0).expect(COORDINATE_ON_TILESET);

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

			let tile = coord.get_from(&tileset.0).expect(COORDINATE_ON_TILESET);

			// Only keep looking beyond a passable tile, and if the current tile is not what we're
			// searching for.
			if tile.is_passable() && tile != needle {
				Adjacent::<Coordinate>::from_array_coordinate(&tileset.0, &coord).for_each(
					|adjacent| {
						let mut new_path = current_path.clone();
						new_path.push(adjacent);

						coordinate_path_queue.push_back((adjacent, new_path))
					},
				);
			}

			// Now that the current coordinate has been fully evaluated, mark it as visited.
			visited.insert(coord, current_path);

			// Using BFS, so if the `tile` is the `needle` we've found the shortest path.
			if tile == needle {
				break;
			}
		}

		visited
			.into_iter()
			.filter(|(coord, _)| {
				// Only want Tiles of `needle`'s type
				coord.get_from(&tileset.0).expect(COORDINATE_ON_TILESET) == needle
			})
			.map(|(_, path)| Path(path))
			.reduce(Path::return_shorter)
	}

	/// # Summary
	///
	/// Find the shortest [`Path`] from some `entrances_or_exits` on a `tileset`.
	///
	/// # Returns
	///
	/// * `Some(Path)` if there is a [`Path`].
	/// * `None` if there is no [`Path`].
	pub fn from_entrances_or_exits(
		tileset: &Tileset,
		entrances_or_exits: &HashSet<Coordinate>,
	) -> Option<Self> {
		entrances_or_exits
			.iter()
			.map(|entrance_or_exit| Path::from_coordinate(tileset, *entrance_or_exit, Tile::Core))
			.flatten()
			.reduce(Path::return_shorter)
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
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, Path, Tile, Tileset, COORDINATE_ON_TILESET, PATH_HAS_COORDINATE},
		crate::map::tileset::tests::{PARK, PARK_TWO_SPAWN},
		std::time::Instant,
	};

	#[test]
	fn from_coordinate() {
		let test_tileset = Tileset(
			PARK.iter()
				.map(|row| row.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_path = Path::from_coordinate(&test_tileset, Coordinate(4, 4), Tile::Core).unwrap();
		println!(
			"Path::from_coordinate {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(test_path.0.len(), 9);
		assert!(test_path.0[..8].into_iter().all(|coord| coord
			.get_from(&test_tileset.0)
			.expect(COORDINATE_ON_TILESET)
			.is_passable()));
		assert!(test_path.0[8]
			.get_from(&test_tileset.0)
			.expect(COORDINATE_ON_TILESET)
			.is_region());
	}

	#[test]
	fn from_entrances_or_exits() {
		let test_tileset = Tileset(
			PARK_TWO_SPAWN
				.iter()
				.map(|row| row.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_paths: Vec<_> = test_tileset
			.entrances()
			.into_iter()
			.map(|entrances| {
				Path::from_entrances_or_exits(&test_tileset, &entrances).expect(PATH_HAS_COORDINATE)
			})
			.collect();
		println!(
			"Path::from_entrances_or_exits {}us",
			Instant::now().duration_since(start).as_micros()
		);

		// There should be two paths to the core since there are two spawn points.
		assert_eq!(test_paths.len(), 2);

		let assertion = |index: usize, desired_len: usize| {
			// Since there may be multiple ways to do this we aren't going to test it
			// directly, rather we're going to assert things about the path instead.
			assert_eq!(test_paths[index].0.len(), desired_len);
			assert!(test_paths[index].0[..(desired_len - 1)]
				.into_iter()
				.all(|coord| coord
					.get_from(&test_tileset.0)
					.expect(COORDINATE_ON_TILESET)
					.is_passable()));
			assert!(test_paths[index].0[desired_len - 1]
				.get_from(&test_tileset.0)
				.expect(COORDINATE_ON_TILESET)
				.is_region());
		};

		// The shortest path from the left-hand Spawn should be of length nine.
		assertion(0, 9);

		// The shortest path from the right-hand Spawn should be of length 15.
		assertion(1, 15);
	}
}
