use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Coordinate, ShortestPath, Tile,
	},
	serde::{Deserialize, Serialize},
	std::collections::HashSet,
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
	/// Get the longest build for a specific `tileset`.
	pub fn from_entrances_to_any_core(tileset: &Tileset, max_blocks: Option<usize>) -> Self {
		let mut build = Build {
			blocks: HashSet::new(),
		};

		let mut current_entrance = 0;
		let mut placements = 0;

		while match max_blocks {
			Some(max) => max > build.blocks.len(),
			_ => true,
		} {
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

			for coord in Vec::<Coordinate>::from(
				ShortestPath::from_any_grid_coordinate_to_tile(
					&tileset.grid,
					Some(&build),
					tileset.entrances_by_region[entrance].iter(),
					Tile::Core,
				)
				.expect(VALID_BUILD),
			)
			.into_iter()
			.rev()
			.filter(|coord| {
				coord.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET) == Tile::Empty
			}) {
				if build.blocks.contains(&coord) {
					continue;
				}

				build.blocks.insert(coord);

				if build.is_valid_for(&tileset) {
					build.try_remove_adjacent_to(&coord, &tileset);
					placements += 1;
					break;
				}

				build.blocks.remove(&coord);
			}
		}

		build
	}

	/// # Summary
	///
	/// Return whether or not the current [`Build`] prevents any entrance from reaching a core.
	fn is_valid_for(&self, tileset: &Tileset) -> bool {
		self.blocks
			.iter()
			.all(|coord| coord.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET) == Tile::Empty)
			&& tileset.entrances_by_region.iter().all(|region| {
				region.iter().any(|entrance| {
					ShortestPath::from_grid_coordinate_to_tile(
						&tileset.grid,
						Some(self),
						*entrance,
						Tile::Core,
					)
					.is_some()
				})
			})
	}

	/// # Summary
	///
	/// Try to remove all coordinates [`Adjacent`] to `coord` on the `tileset`, and see if removing
	/// them from this [`Build`] would alter the [`ShortestPath::from_entrances_to_any_core`].
	fn try_remove_adjacent_to(&mut self, coord: &Coordinate, tileset: &Tileset) {
		let mut expected_shortest_path = None;

		Adjacent::<Coordinate>::from_grid_coordinate(&tileset.grid, &coord).for_each(|adjacent| {
			if self.blocks.contains(&adjacent) {
				if expected_shortest_path.is_none() {
					expected_shortest_path = Some(ShortestPath::from_entrances_to_any_core(
						&tileset,
						Some(&self),
					));
				}

				self.blocks.remove(&adjacent);

				let actual_shortest_path =
					ShortestPath::from_entrances_to_any_core(&tileset, Some(&self));

				if &actual_shortest_path
					!= expected_shortest_path
						.as_ref()
						.expect("Expected `shortest_path` to be `Some` by now")
				{
					self.blocks.insert(adjacent);
				}
			}
		});
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
	fn from_entrances_to_any_core() {
		let mut test_tileset = Tileset::new(
			PARK_TWO_SPAWN
				.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let build = Build::from_entrances_to_any_core(&test_tileset, None);
		println!(
			"Build::from_entrances_to_any_core {}us",
			Instant::now().duration_since(start).as_micros()
		);

		build.apply_to(&mut test_tileset.grid);
		println!("{:?}", test_tileset.grid);

		// TODO: write assertions
	}

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
		assert!(Build {
			blocks: HashSet::new()
		}
		.is_valid_for(&test_tileset));

		// Valid build for valid tileset.
		assert!(Build {
			blocks: [Coordinate(4, 1)].iter().copied().collect()
		}
		.is_valid_for(&test_tileset));

		// Invalid build for valid tileset (because of no path)
		assert!(!Build {
			blocks: [
				Coordinate(4, 1),
				Coordinate(5, 2),
				Coordinate(5, 3),
				Coordinate(5, 4),
				Coordinate(5, 5),
				Coordinate(4, 6),
			]
			.iter()
			.copied()
			.collect()
		}
		.is_valid_for(&test_tileset));

		// Invalid build for valid tileset (because of invalid block placement).
		assert!(!Build {
			blocks: [Coordinate(0, 1)].iter().copied().collect()
		}
		.is_valid_for(&test_tileset));

		println!(
			"Build::is_valid_for {}us",
			Instant::now().duration_since(start).as_micros() / 4
		);
	}
}
