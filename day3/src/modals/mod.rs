mod test;

/// coordiantes {x,y}
/// ---
/// Description - Vector-2d are i64 2d coordinate type {x,y}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Vec2 {
    /// Converts i64 tuple to Vec2 type.
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Open
    }
}