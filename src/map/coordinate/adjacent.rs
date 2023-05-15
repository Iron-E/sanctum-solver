mod from_coordinate;

use super::Coordinate;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adjacent {
	top: Option<Coordinate>,
	right: Coordinate,
	down: Coordinate,
	left: Option<Coordinate>,
}
