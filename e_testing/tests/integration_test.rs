mod common;

use e_testing;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, e_testing::add_two(2));
}
