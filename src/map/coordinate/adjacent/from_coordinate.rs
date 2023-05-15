use {super::Adjacent, crate::map::Coordinate};

impl From<Coordinate> for Adjacent {
	fn from(other: Coordinate) -> Self {
		Self {
			top: if other.1 > 0 {
				Some(Coordinate(other.0, other.1 - 1))
			} else {
				None
			},
			right: Coordinate(other.0 + 1, other.1),
			down: Coordinate(other.0, other.1 + 1),
			left: if other.0 > 0 {
				Some(Coordinate(other.0 - 1, other.1))
			} else {
				None
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Adjacent, Coordinate};

	#[test]
	fn from() {
		assert_eq!(
			// Normal adjacency; no special cases
			Adjacent::from(Coordinate(2, 2)),
			Adjacent {
				top: Some(Coordinate(2, 1)),
				right: Coordinate(3, 2),
				down: Coordinate(2, 3),
				left: Some(Coordinate(1, 2)),
			}
		);

		assert_eq!(
			// Nothing to the left.
			Adjacent::from(Coordinate(0, 2)),
			Adjacent {
				top: Some(Coordinate(0, 1)),
				right: Coordinate(1, 2),
				down: Coordinate(0, 3),
				left: None,
			}
		);

		assert_eq!(
			// Nothing to the top.
			Adjacent::from(Coordinate(2, 0)),
			Adjacent {
				top: None,
				down: Coordinate(2, 1),
				right: Coordinate(3, 0),
				left: Some(Coordinate(1, 0)),
			}
		);

		assert_eq!(
			// Nothing to the top or left.
			Adjacent::from(Coordinate(0, 0)),
			Adjacent {
				top: None,
				right: Coordinate(1, 0),
				down: Coordinate(0, 1),
				left: None,
			}
		);
	}
}
