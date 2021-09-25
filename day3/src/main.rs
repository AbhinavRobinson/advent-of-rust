mod modals;
use modals::{Map, Tile, Vec2};

fn main() {
    let map = Map::parse(include_bytes!("input.txt"));
    let itinerary = (0..map.size.y).into_iter().map(|y| Vec2::from((y * 3, y)));
    let num_trees = itinerary.filter(|&pos| map.get(pos) == Tile::Tree).count();
    println!("We encountered {} trees", num_trees);
}
