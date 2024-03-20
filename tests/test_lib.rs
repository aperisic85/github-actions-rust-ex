use trust::*;

#[test]
fn test_add() {
    assert_eq!(add(2,2), 4);
}
#[test]
fn test_sub() {
    assert_eq!(sub(2, 2), 0);
}
#[test]
fn test_mul() {
    assert_eq!(mul(2, 2), 4);
}
#[test]
fn test_div() {
    assert_eq!(div(2, 2), 1);
}