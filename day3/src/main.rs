mod modals;
use modals::Map;

fn main() {
    let map = Map::parse(include_bytes!("input.txt"));
    dbg!(map.size);
    println!("{:?}", map);
}
