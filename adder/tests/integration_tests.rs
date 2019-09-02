use adder;
mod common;
// Integration tests can't be created if no library
//Can create multiple crates in tests folder
#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, adder::add_two(2));
}