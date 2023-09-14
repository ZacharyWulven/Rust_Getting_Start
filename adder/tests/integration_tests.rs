// 对 lib.rs 进行测试
// lib.rs 在本模块中，本模块名为 adder
use adder;

// 导入 common 模块
mod common;

#[test]
fn it_adds_two() {
    // common::setup();
    assert_eq!(4, adder::add_two(2));
}