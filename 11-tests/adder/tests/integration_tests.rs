use adder;
// Using the common module (subdir)
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add(2, 2));
}
