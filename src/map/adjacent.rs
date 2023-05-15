use {
	super::{Build, Coordinate, Tile},
	std::{array::IntoIter, iter::Flatten},
};

/// # Summary
///
/// Types which are adjacent to some other type.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adjacent<T> {
	pub up: Option<T>,
	pub up_right: Option<T>,
	pub right: Option<T>,
	pub down_right: Option<T>,
	pub down: Option<T>,
	pub down_left: Option<T>,
	pub left: Option<T>,
	pub up_left: Option<T>,
}

impl<T> Adjacent<T> {
	/// # Summary
	///
	/// Run some `f`unction on each [`Some`] value.
	pub fn for_each(self, mut f: impl FnMut(T)) {
		/// # Summary
		///
		/// Makes calling the passed in `f` function more simple than writing `if let` four times.
		macro_rules! call {
			($arg: expr) => {
				if let Some(some_arg) = $arg {
					f(some_arg);
				}
			};
		}

		call!(self.up);
		call!(self.up_right);
		call!(self.right);
		call!(self.down_right);
		call!(self.down);
		call!(self.down_left);
		call!(self.left);
		call!(self.up_left);
	}
}

impl Adjacent<Coordinate> {
	/// # Summary
	///
	/// Get the adjacent [`Coordinate`]s to a `coordinate` on an `array`.
	pub fn from_grid_coordinate<T>(grid: &[impl AsRef<[T]>], coord: &Coordinate) -> Self {
		/// # Summary
		///
		/// If `$cond` is `true`, then return `Some($value)`. Otherwise, return `None`.
		macro_rules! if_then_or_none {
			($cond: expr, $value: expr) => {
				if $cond {
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
			up_right: if_then_or_none!(
				can_move_up && can_move_right,
				Coordinate(coord.0 + 1, coord.1 - 1)
			),
			right: if_then_or_none!(can_move_right, Coordinate(coord.0 + 1, coord.1)),
			down_right: if_then_or_none!(
				can_move_down && can_move_right,
				Coordinate(coord.0 + 1, coord.1 + 1)
			),
			down: if_then_or_none!(can_move_down, Coordinate(coord.0, coord.1 + 1)),
			down_left: if_then_or_none!(
				can_move_down && can_move_left,
				Coordinate(coord.0 - 1, coord.1 + 1)
			),
			left: if_then_or_none!(can_move_left, Coordinate(coord.0 - 1, coord.1)),
			up_left: if_then_or_none!(
				can_move_up && can_move_left,
				Coordinate(coord.0 - 1, coord.1 - 1)
			),
		}
	}

	/// # Summary
	///
	/// Return [`Self::from_grid_coordinate`] but with blocked diagonals reflecteed from the build.
	pub fn from_build_coordinate(
		grid: &[impl AsRef<[Tile]>],
		build: Option<&Build>,
		coord: &Coordinate,
	) -> Self {
		let mut adjacents = Self::from_grid_coordinate(grid, coord);

		if let Some(b) = build {
			let contains_up = adjacents
				.up
				.and_then(|up| Some(b.blocks.contains(&up)))
				.unwrap_or(false);
			let contains_right = adjacents
				.right
				.and_then(|right| Some(b.blocks.contains(&right)))
				.unwrap_or(false);
			let contains_down = adjacents
				.down
				.and_then(|down| Some(b.blocks.contains(&down)))
				.unwrap_or(false);
			let contains_left = adjacents
				.left
				.and_then(|left| Some(b.blocks.contains(&left)))
				.unwrap_or(false);

			/// # Summary
			///
			/// If `$cond` is `true`, then return `Some($value)`. Otherwise, return `None`.
			macro_rules! if_then_none {
				($($cond: expr)*, $field: ident) => {
					if $($cond)&&* {
						adjacents.$field = None;
					}
				};
			}

			if_then_none!(contains_up contains_right, up_right);
			if_then_none!(contains_down contains_right, down_right);
			if_then_none!(contains_down contains_left, down_left);
			if_then_none!(contains_up contains_left, up_left);
		}

		adjacents
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Adjacent, Coordinate},
		crate::map::{Tile, Tile::*},
	};

	#[rustfmt::skip]
	const ARRAY: [[Tile; 5]; 5] = [
		[Impass, Impass, Impass, Impass, Impass],
		[Spawn,  Pass,   Empty,  Core,   Impass],
		[Spawn,  Pass,   Empty,  Core,   Impass],
		[Spawn,  Pass,   Empty,  Core,   Impass],
		[Impass, Impass, Impass, Impass, Impass],
	];

	#[test]
	fn from_array_coordinate() {
		// Normal adjacency; no special cases
		assert_eq!(
			Adjacent::<Coordinate>::from_grid_coordinate(&ARRAY, &Coordinate(2, 2)),
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
			Adjacent::<Coordinate>::from_grid_coordinate(&ARRAY, &Coordinate(2, 0)),
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
			Adjacent::<Coordinate>::from_grid_coordinate(&ARRAY, &Coordinate(4, 3)),
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
			Adjacent::<Coordinate>::from_grid_coordinate(&ARRAY, &Coordinate(3, 4)),
			Adjacent {
				up: Some(Coordinate(3, 3)),
				up_right: Some(Coordinate(4, 3)),
				right: Some(Coordinate(4, 4)),
				down_right: None,
				down: None,
				down_left: None,
				left: Some(Coordinate(2, 4)),
				up_left: Some(Coordinate(2, 3)),
			}
		);

		// Nothing to the left.
		assert_eq!(
			Adjacent::<Coordinate>::from_grid_coordinate(&ARRAY, &Coordinate(0, 2)),
			Adjacent {
				up: Some(Coordinate(0, 1)),
				up_right: Some(Coordinate(1, 1)),
				right: Some(Coordinate(1, 2)),
				down_right: Some(Coordinate(1, 3)),
				down: Some(Coordinate(0, 3)),
				down_left: None,
				left: None,
				up_left: None,
			}
		);
	}
}
