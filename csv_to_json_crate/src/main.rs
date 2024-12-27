// use csv_to_json_crate::models;

fn main() {
    println!("Hello, world!");
}

/*
    rust 会从三个地方找这个 modules 模块
    1. inline 模式：声明模块的地方是否有大括号
    2. 声明到 modules.rs 里
    3. 找 models/mod.rs 文件
 */
mod models;