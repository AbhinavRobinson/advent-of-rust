use core::panic;

mod test;

/// coordiantes {x,y}
/// ---
/// Vector-2d are i64 2d coordinate type {x,y}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
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
pub enum Tile {
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
pub struct Map {
    pub size: Vec2,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    pub fn set(&mut self, pos: Vec2, tile: Tile) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile
        }
    }

    pub fn get(&self, pos: Vec2) -> Tile {
        self.index(pos).map(|i| self.tiles[i]).unwrap_or_default()
    }

    pub fn normalize_pos(&self, pos: Vec2) -> Option<Vec2> {
        if pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            let x = pos.x % self.size.x;
            let x = if x < 0 { self.size.x + x } else { x };
            Some((x, pos.y).into())
        }
    }

    pub fn index(&self, pos: Vec2) -> Option<usize> {
        self.normalize_pos(pos)
            .map(|pos| (pos.x + pos.y * self.size.x) as _)
    }

    pub fn parse(input: &[u8]) -> Self {
        let mut cols = 0;
        let mut rows = 1;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                cols = 0;
            } else {
                cols += 1;
            }
        }

        let mut iter = input.iter().copied();
        let mut map = Self::new((cols, rows).into());
        for row in 0..map.size.y {
            for col in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Open,
                    Some(b'#') => Tile::Tree,
                    c => panic!("Expected '.' or '#', but got: {:?}", c),
                };
                map.set((col, row).into(), tile);
            }
            iter.next();
        }
        map
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
