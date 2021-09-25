#[cfg(test)]
use super::{Map, Vec2};

#[test]
fn test_tuple() {
    let v: Vec2 = (5, 8).into();
    assert_eq!(v.x, 5);
    assert_eq!(v.y, 8);
}

#[test]
fn test_normalize_pos() {
    let m = Map::new((2, 2).into());
    assert_eq!(m.normalize_pos((0, 0).into()), Some((0, 0).into()));
    assert_eq!(m.normalize_pos((1, 0).into()), Some((1, 0).into()));
    assert_eq!(m.normalize_pos((2, 0).into()), Some((0, 0).into()));
    assert_eq!(m.normalize_pos((-1, 0).into()), Some((1, 0).into()));
    assert_eq!(m.normalize_pos((-2, 0).into()), Some((0, 0).into()));
    assert_eq!(m.normalize_pos((0, -1).into()), None);
    assert_eq!(m.normalize_pos((0, 2).into()), None);
}

#[test]
fn test_index() {
    let m = Map::new((3, 5).into());
    assert_eq!(m.index((0, 0).into()), Some(0));
    assert_eq!(m.index((2, 0).into()), Some(2));
    assert_eq!(m.index((0, 1).into()), Some(3));
    assert_eq!(m.index((2, 1).into()), Some(5));
}
