use super::*;

#[test]
fn coord() {
    assert!(Coord::new(5, 5).is_some());
    assert!(Coord::new(7, 0).is_some());
    assert!(Coord::new(0, 7).is_some());
    assert!(Coord::new(9, 0).is_none());
    assert!(Coord::new(0, 12).is_none());
    assert!(Coord::new(264, 12).is_none());
}
