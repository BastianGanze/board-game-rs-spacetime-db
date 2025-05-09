use std::fmt::{Display, Formatter};
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Coord<const X: u8, const Y: u8> {
    index: u8,
}

pub type Coord3 = Coord<3, 3>;
pub type Coord8 = Coord<8, 8>;

pub type CoordAllIter<C> = std::iter::Map<std::ops::Range<u8>, fn(u8) -> C>;

impl<const X: u8, const Y: u8> Coord<X, Y> {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < X * Y);
        Coord { index }
    }

    pub const fn from_xy(x: u8, y: u8) -> Self {
        assert!(x < X);
        assert!(y < Y);
        Coord { index: x + X * y }
    }

    pub fn all() -> CoordAllIter<Self> {
        (0..X * Y).map(Coord::from_index)
    }

    pub const fn index(self) -> u8 {
        self.index
    }

    pub const fn dense_index(self, size: u8) -> usize {
        size as usize * self.y() as usize + self.x() as usize
    }

    pub const fn x(self) -> u8 {
        self.index % X
    }

    pub const fn y(self) -> u8 {
        self.index / X
    }

    pub const fn manhattan_distance(self, other: Coord<X, Y>) -> u8 {
        let dx = self.x().abs_diff(other.x());
        let dy = self.y().abs_diff(other.y());
        dx + dy
    }

    pub const fn diagonal_distance(self, other: Coord<X, Y>) -> u8 {
        let dx = self.x().abs_diff(other.x());
        let dy = self.y().abs_diff(other.y());

        // max is not const yet
        if dx >= dy {
            dx
        } else {
            dy
        }
    }

    pub const fn cast<const X2: u8, const Y2: u8>(self) -> Coord<X2, Y2> {
        Coord::<X2, Y2>::from_xy(self.x(), self.y())
    }

    pub const fn valid_for_size(self, size: u8) -> bool {
        self.x() < size && self.y() < size
    }
}

impl<const X: u8, const Y: u8> Display for Coord<X, Y> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}
