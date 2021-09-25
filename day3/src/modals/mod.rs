mod test;

/// coordiantes {x,y}
/// ---
/// Vector-2d are i64 2d coordinate type {x,y}
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

/// tile '.' || '#' for Open || Tree
/// ---
/// Tile of map which can either be tree or empty.
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

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Open => '.',
            Tile::Tree => '#',
        };
        write!(f, "{}", c)
    }
}

/// Map - Contains tiles of Tree || Empty Space
/// ---
/// Map type for set of tiles from input
struct Map {
    size: Vec2,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn set(&self, pos: Vec2, tile: Tile) {
        todo!()
    }

    fn get(&self, pos: Vec2) -> Tile {
        todo!()
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get((col, row).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
