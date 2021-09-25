mod modals;
use modals::{Map, Tile};

fn main() {
    let map = {
        let mut m = Map::new((6, 6).into());
        let points = [(1, 1), (4, 1), (1, 3), (4, 3), (2, 4), (3, 4)];
        for p in (&points).iter().copied() {
            m.set(p.into(), Tile::Tree);
        }
        m
    };
    println!("{:?}", map);
}
