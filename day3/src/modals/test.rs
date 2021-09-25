#[cfg(test)]
use super::Vec2;

#[test]
fn test_tuple() {
    let v: Vec2 = (5, 8).into();
    assert_eq!(v.x, 5);
    assert_eq!(v.y, 8);
}
