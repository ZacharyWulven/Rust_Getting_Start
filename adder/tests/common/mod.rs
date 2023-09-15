

// 这是一个 helper 或 wrapper 功能的代码文件
// 其不会被 Rust 视为一个集成测试文件

pub fn setup() {
    println!("common setup");
    panic!("common panic");
}