use {
	super::{tileset::COORDINATE_ON_TILESET, Coordinate, Tile},
	crate::Container,
};

/// # Summary
///
/// Types which are adjacent to some other type.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adjacent<T> {
	pub up: Option<T>,
	pub right: Option<T>,
	pub down: Option<T>,
	pub left: Option<T>,

	pub up_right: Option<T>,
	pub down_right: Option<T>,
	pub down_left: Option<T>,
	pub up_left: Option<T>,
}

impl<T> Adjacent<T> {
	/// # Summary
	///
	/// Run some `f`unction on each [`Some`] value.
	pub fn for_each(self, mut f: impl FnMut(T)) {
		/// # Summary
		///
		/// Call `f` on `$arg`
		macro_rules! call_if_some {
			($arg: expr) => {
				if let Some(some_arg) = $arg {
					f(some_arg);
				}
			};
		}

		call_if_some!(self.up);
		call_if_some!(self.right);
		call_if_some!(self.down);
		call_if_some!(self.left);

		call_if_some!(self.up_right);
		call_if_some!(self.down_right);
		call_if_some!(self.down_left);
		call_if_some!(self.up_left);
	}
}

impl Adjacent<Coordinate> {
	/// # Summary
	///
	/// Return [`Self::from_grid_coordinate`] but with blocked diagonals reflecteed from the build.
	/// # Summary
	///
	/// Get the adjacent [`Coordinate`]s to a `coordinate` on an `array`.
	pub fn from_grid_coordinate<T>(grid: &[impl AsRef<[T]>], coord: &Coordinate, diagonals: bool) -> Self {
		/// # Summary
		///
		/// If `$cond` is `true`, then return `Some($value)`. Otherwise, return `None`.
		macro_rules! if_then_or_none {
			($($cond: expr)+, $value: expr) => {
				if $($cond)&&* {
					Some($value)
				} else {
					None
				}
			};
		}

		let can_move_up = coord.1 > 0;
		let can_move_right = coord.0 < grid[coord.1].as_ref().len() - 1;
		let can_move_down = coord.1 < grid.len() - 1;
		let can_move_left = coord.0 > 0;

		Self {
			up: if_then_or_none!(can_move_up, Coordinate(coord.0, coord.1 - 1)),
			right: if_then_or_none!(can_move_right, Coordinate(coord.0 + 1, coord.1)),
			down: if_then_or_none!(can_move_down, Coordinate(coord.0, coord.1 + 1)),
			left: if_then_or_none!(can_move_left, Coordinate(coord.0 - 1, coord.1)),

			up_right: if_then_or_none!(
				can_move_up can_move_right diagonals,
				Coordinate(coord.0 + 1, coord.1 - 1)
			),
			down_right: if_then_or_none!(
				can_move_down can_move_right diagonals,
				Coordinate(coord.0 + 1, coord.1 + 1)
			),
			down_left: if_then_or_none!(
				can_move_down can_move_left diagonals,
				Coordinate(coord.0 - 1, coord.1 + 1)
			),
			up_left: if_then_or_none!(
				can_move_up can_move_left diagonals,
				Coordinate(coord.0 - 1, coord.1 - 1)
			),
		}
	}

	pub fn from_grid_coordinate_with_build(
		grid: &[impl AsRef<[Tile]>],
		build: Option<&impl Container<Coordinate>>,
		coord: &Coordinate,
		diagonals: bool,
	) -> Self {
		let mut adjacent = Self::from_grid_coordinate(grid, coord, diagonals);

		/// # Summary
		///
		/// If `$cond` is `true`, then return `Some($value)`. Otherwise, return `None`.
		///
		/// # Remarks
		///
		/// We don't set it to `Impass` or `Block`, because `None`s are ignored by `for_each`.
		/// Therefore we get a performance improvement.
		macro_rules! if_then_none {
			($($cond: expr)+, $field: ident) => {
				if $(!$cond)&&* {
					adjacent.$field = None;
				}
			};
		}

		if diagonals {
			let can_move_to = |direction: Option<Coordinate>| -> bool {
				direction
					.map(|d| {
						d.get_from_with_build(&grid, build)
							.expect(COORDINATE_ON_TILESET)
							.is_passable()
					})
					.unwrap_or(false)
			};

			let can_move_up = can_move_to(adjacent.up);
			let can_move_right = can_move_to(adjacent.right);
			let can_move_down = can_move_to(adjacent.down);
			let can_move_left = can_move_to(adjacent.left);

			if_then_none!(can_move_up can_move_right, up_right);
			if_then_none!(can_move_down can_move_right, down_right);
			if_then_none!(can_move_down can_move_left, down_left);
			if_then_none!(can_move_up can_move_left, up_left);
		}

		adjacent
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Adjacent, Coordinate},
		crate::map::{Build, Tile, Tile::*},
		std::time::Instant,
	};

	#[rustfmt::skip]
	const ARRAY: [[Tile; 5]; 5] = [
		// 0    1      2      3      4
		[Empty, Empty,  Empty, Empty, Empty], // 0
		[Spawn, Empty,  Empty, Empty, Core],  // 1
		[Spawn, Impass, Empty, Empty, Core],  // 2
		[Spawn, Empty,  Empty, Empty, Core],  // 3
		[Empty, Empty,  Empty, Empty, Core],  // 4
	];

	#[test]
	fn from_grid_coordinate() {
		let start = Instant::now();

		// Normal adjacency; no special cases
		assert_eq!(
			Adjacent::from_grid_coordinate(&ARRAY, &Coordinate(2, 2), true),
			Adjacent {
				up: Some(Coordinate(2, 1)),
				up_right: Some(Coordinate(3, 1)),
				right: Some(Coordinate(3, 2)),
				down_right: Some(Coordinate(3, 3)),
				down: Some(Coordinate(2, 3)),
				down_left: Some(Coordinate(1, 3)),
				left: Some(Coordinate(1, 2)),
				up_left: Some(Coordinate(1, 1)),
			}
		);

		// Nothing to the top.
		assert_eq!(
			Adjacent::from_grid_coordinate(&ARRAY, &Coordinate(2, 0), true),
			Adjacent {
				up: None,
				up_right: None,
				right: Some(Coordinate(3, 0)),
				down_right: Some(Coordinate(3, 1)),
				down: Some(Coordinate(2, 1)),
				down_left: Some(Coordinate(1, 1)),
				left: Some(Coordinate(1, 0)),
				up_left: None,
			}
		);

		// Nothing to the right.
		assert_eq!(
			Adjacent::from_grid_coordinate(&ARRAY, &Coordinate(4, 3), true),
			Adjacent {
				up: Some(Coordinate(4, 2)),
				up_right: None,
				right: None,
				down_right: None,
				down: Some(Coordinate(4, 4)),
				down_left: Some(Coordinate(3, 4)),
				left: Some(Coordinate(3, 3)),
				up_left: Some(Coordinate(3, 2)),
			}
		);

		// Nothing to the bottom.
		assert_eq!(
			Adjacent::from_grid_coordinate(&ARRAY, &Coordinate(3, 4), true),
			Adjacent {
				up: Some(Coordinate(3, 3)),
				right: Some(Coordinate(4, 4)),
				down: None,
				left: Some(Coordinate(2, 4)),

				up_right: Some(Coordinate(4, 3)),
				down_right: None,
				down_left: None,
				up_left: Some(Coordinate(2, 3)),
			}
		);

		// Nothing to the left.
		assert_eq!(
			Adjacent::from_grid_coordinate(&ARRAY, &Coordinate(0, 2), true),
			Adjacent {
				up: Some(Coordinate(0, 1)),
				right: Some(Coordinate(1, 2)),
				down: Some(Coordinate(0, 3)),
				left: None,

				up_right: Some(Coordinate(1, 1)),
				down_right: Some(Coordinate(1, 3)),
				down_left: None,
				up_left: None,
			}
		);

		println!(
			"Adjacent::from_grid_coordinate {}us",
			Instant::now().duration_since(start).as_micros() / 6
		);
	}

	#[test]
	fn from_grid_coordinate_with_build() {
		let build = Build {
			blocks: [Coordinate(2, 1), Coordinate(3, 2)]
				.iter()
				.copied()
				.collect(),
		};

		let start = Instant::now();
		let adjacent = Adjacent::from_grid_coordinate_with_build(
			&ARRAY,
			Some(&build.blocks),
			&Coordinate(2, 2),
			true,
		);
		println!(
			"Adjacent::from_grid_coordinate_with_build {}us",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(
			adjacent,
			Adjacent {
				up: Some(Coordinate(2, 1)),
				right: Some(Coordinate(3, 2)),
				down: Some(Coordinate(2, 3)),
				left: Some(Coordinate(1, 2)),

				up_right: None,
				down_right: Some(Coordinate(3, 3)),
				down_left: Some(Coordinate(1, 3)),
				up_left: None,
			},
		);
	}
}
