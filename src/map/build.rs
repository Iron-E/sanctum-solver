mod temp_build;

use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Coordinate, ShortestPath, Tile,
	},
	crate::Container,
	rayon::iter::IntoParallelRefIterator,
	serde::{Deserialize, Serialize},
	std::collections::{BTreeMap, HashSet, LinkedList},
	temp_build::TempBuild,
};

const VALID_BUILD: &str = "Expected build to produce shortest paths";

/// # Summary
///
/// A set of blocks for a [`Tileset`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Build {
	pub blocks: HashSet<Coordinate>,
}

impl Build {
	/// # Summary
	///
	/// Apply all of the `blocks` from the [`Build`] to a `tileset`.
	pub fn apply_to(&self, grid: &mut [impl AsMut<[Tile]>]) {
		self.blocks.iter().for_each(|coordinate| {
			coordinate.set(grid, Tile::Block);
		})
	}

	/// # Summary
	///
	/// Finds a [valid][valid] [block][block] placement closest to the [`Tile::Core`].
	///
	/// # Parameters
	///
	/// * `tileset`, the [`Tileset`] this [block][block] is being placed on.
	/// * `blocks`, the previously placed [block][block]s.
	/// * `shortest_path`, the current shortest path through the `blocks`.
	///
	/// # Returns
	///
	/// * `None`, if no block can be placed along `shortest_path`.
	/// * `Some(Coordinate)`, detailing where a block can be placed which is still [valid][valid].
	///
	/// [block]: Tile::Block
	/// [valid]: Build::is_valid
	pub fn find_valid_block_placement(
		tileset: &Tileset,
		blocks: &impl Container<Coordinate>,
		shortest_path: Vec<Coordinate>,
	) -> Option<Coordinate> {
		shortest_path
			.into_iter()
			.rev()
			.find(|coord| {
				// We only want empty tiles
				coord.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET) == Tile::Empty
					&& Build::is_valid(
						&tileset,
						&TempBuild {
							blocks,
							temp_block: *coord,
						},
					)
			})
	}

	/// # Summary
	///
	/// Get the longest build for a specific `tileset` by using round-robin on all of the spawn
	/// regions.
	///
	/// # Parameters
	///
	/// * `diagonals`, whether to use diagonal movement.
	/// * `max_blocks`, the maximum number of blocks to place.
	pub fn from_entrances_to_any_core(
		tileset: &Tileset,
		diagonals: bool,
		max_blocks: Option<usize>,
	) -> Self {
		let mut build = Build {
			blocks: HashSet::new(),
		};

		let mut current_entrance = 0;
		let mut placements = 1;

		while max_blocks
			.map(|max| max > build.blocks.len())
			.unwrap_or(true)
		{
			let entrance = {
				// If we're still iterating over the number of entrances
				if current_entrance < tileset.entrances_by_region.len() - 1 {
					current_entrance += 1;
				// If blocks are still being placed.
				} else if placements > 0 {
					current_entrance = 0;
					placements = 0;
				} else {
					break;
				}
				current_entrance
			};

			if let Some(coord) = Build::find_valid_block_placement(
				tileset,
				&build.blocks,
				ShortestPath::from_any_grid_coordinate_to_tile(
					&tileset.grid,
					Some(&build.blocks),
					tileset.entrances_by_region[entrance].par_iter(),
					Tile::Core,
					diagonals,
				)
				.expect(VALID_BUILD)
				.into(),
			) {
				// Test the build with the coordinate inserted.
				// Insert the coord now that we know it is valid.
				build.blocks.insert(coord);
				build.try_remove_adjacent_to(&tileset, coord, diagonals);

				// Mark the block as having been placed.
				placements += 1;
			}
		}

		build
	}

	/// # Summary
	///
	/// Get the longest build for a specific `tileset` by taking priority on the current shortest
	/// path.
	pub fn from_entrances_to_any_core_with_priority(
		tileset: &Tileset,
		diagonals: bool,
		max_blocks: Option<usize>,
	) -> Self {
		let mut build = Build {
			blocks: HashSet::new(),
		};

		let mut shortest_paths_by_region: BTreeMap<_, _> =
			ShortestPath::from_entrances_to_any_core(
				&tileset,
				Option::<&HashSet<_>>::None,
				diagonals,
			)
			.into_iter()
			.enumerate()
			.map(|(index, shortest_path)| (shortest_path.expect(VALID_BUILD), index))
			.collect();

		while let Some((shortest_path, region_index)) = shortest_paths_by_region.pop_first() {
			// Make sure we have less than the maximum blocks.
			if max_blocks
				.map(|max| build.blocks.len() >= max)
				.unwrap_or(false)
			{
				break;
			}

			/// # Summary
			///
			/// Create a new shortest path.
			macro_rules! shortest_path {
				() => {
					ShortestPath::from_any_grid_coordinate_to_tile(
						&tileset.grid,
						Some(&build.blocks),
						tileset.entrances_by_region[region_index].par_iter(),
						Tile::Core,
						diagonals,
					)
					.expect(VALID_BUILD)
				};
			}

			let shortest_path_vec = Vec::from(shortest_path);

			// The shortest path for this region has had a block placed over it. Recalculate and try again!
			if shortest_path_vec
				.iter()
				.any(|coord| build.blocks.contains(coord))
			{
				shortest_paths_by_region.insert(shortest_path!(), region_index);
				continue;
			}

			if let Some(coord) =
				Build::find_valid_block_placement(tileset, &build.blocks, shortest_path_vec)
			{
				// It was valid, so insert it.
				build.blocks.insert(coord);
				build.try_remove_adjacent_to(&tileset, coord, diagonals);

				// Recalculate the shortest path as well.
				shortest_paths_by_region.insert(shortest_path!(), region_index);
			}
		}

		build
	}

	/// # Summary
	///
	/// Return whether or not the current [`Build`] prevents any entrance from reaching a core.
	fn is_valid(tileset: &Tileset, blocks: &impl Container<Coordinate>) -> bool {
		// A valid build only contains coordinates which are for `Empty` tiles
		tileset.entrances_by_region.iter().all(|region| {
			// Additionally, there should be at least one entrance in every region which has a path to a core.
			region.iter().any(|entrance| {
				ShortestPath::from_grid_coordinate_to_tile(
					&tileset.grid,
					Some(blocks),
					*entrance,
					Tile::Core,
					false,
				)
				.is_some()
			})
		})
	}

	/// # Summary
	///
	/// Try to remove all coordinates [`Adjacent`] to `coord` on the `tileset`, and see if removing
	/// them from this [`Build`] would alter the [`ShortestPath::from_entrances_to_any_core`].
	///
	/// Returns `true` if an item was returned.
	fn try_remove_adjacent_to(&mut self, tileset: &Tileset, coord: Coordinate, diagonals: bool) {
		// Lazy load the expected shortest paths. We may not need to calculate it!
		let mut expected_shortest_paths = None;

		// Which coordinates we have already tried removing.
		let mut visited = HashSet::<Coordinate>::new();

		// Queue of `Adjacent`s we want to try.
		let mut adjacent_queue = LinkedList::new();
		adjacent_queue.push_back(Adjacent::from_grid_coordinate(
			&tileset.grid,
			&coord,
			diagonals,
		));

		while let Some(adjacent) = adjacent_queue.pop_front() {
			adjacent.for_each(|adjacent_coord| {
				if self.blocks.contains(&adjacent_coord) && !visited.contains(&adjacent_coord) {
					// Mark this coordinate as visited.
					visited.insert(adjacent_coord);

					// We'll need this value to be `Some`thing now.
					if expected_shortest_paths.is_none() {
						expected_shortest_paths = Some(ShortestPath::from_entrances_to_any_core(
							&tileset,
							Some(&self.blocks),
							diagonals,
						));
					}

					// If a coordinate was removed,
					if self.try_remove_coord(
						tileset,
						expected_shortest_paths
							.as_ref()
							.expect("Expected `shortest_path` to be `Some` by now"),
						coord,
						diagonals,
					) {
						// Look at adjacent coordinates to see if any of those can be removed either.
						adjacent_queue.push_back(Adjacent::from_grid_coordinate(
							&tileset.grid,
							&adjacent_coord,
							diagonals,
						));
					}
				}
			});
		}
	}

	/// # Summary
	///
	/// See if removing `coord` them from this [`Build`]  would alter the [`ShortestPath::from_entrances_to_any_core`], and if it wouldn't remove it.
	///
	/// Returns `true` if an item was removed.
	fn try_remove_coord(
		&mut self,
		tileset: &Tileset,
		expected_shortest_paths: &[Option<ShortestPath>],
		coord: Coordinate,
		diagonals: bool,
	) -> bool {
		// If the coordinate was removed (and therefore part of the build in the first place)
		if self.blocks.remove(&coord) {
			let actual_shortest_path =
				ShortestPath::from_entrances_to_any_core(&tileset, Some(&self.blocks), diagonals);

			// If it changed ANYTHING about the shortest paths
			if actual_shortest_path != expected_shortest_paths {
				self.blocks.insert(coord);
				return false;
			}

			// Wasn't needed, return true.
			return true;
		}

		// Nothing happened, return false.
		false
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Build, Coordinate, HashSet, Tileset},
		crate::map::tileset::tests::PARK_TWO_SPAWN,
		std::time::Instant,
	};

	#[test]
	fn is_valid() {
		let test_tileset = Tileset::new(
			PARK_TWO_SPAWN
				.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();

		// Empty build should be valid for a valid Tileset.
		assert!(Build::is_valid(&test_tileset, &HashSet::new()));

		// Valid build for valid tileset.
		assert!(Build::is_valid(
			&test_tileset,
			&[Coordinate(4, 1)].iter().copied().collect::<HashSet<_>>()
		));

		// Invalid build for valid tileset (because of no path)
		assert!(!Build::is_valid(
			&test_tileset,
			&[
				Coordinate(4, 1),
				Coordinate(5, 2),
				Coordinate(5, 3),
				Coordinate(5, 4),
				Coordinate(5, 5),
				Coordinate(4, 6),
			]
			.iter()
			.copied()
			.collect::<HashSet<_>>(),
		));

		println!(
			"Build::is_valid {}us",
			Instant::now().duration_since(start).as_micros() / 3
		);
	}
}
