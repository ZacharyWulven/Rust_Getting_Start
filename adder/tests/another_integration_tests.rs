// 对 lib.rs 进行测试
// lib.rs 在本模块中，本模块名为 adder
use adder;

#[test]
fn it_another_adds_two() {
    assert_eq!(4, adder::add_two(2));
}