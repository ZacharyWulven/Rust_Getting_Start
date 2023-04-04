/*
    用于读取命令行参数，由标准库提供
    args 返回一个迭代器，迭代器返回一系列值，调用 collect 可将其转化为一个集合
    std::env::args()
 */
use std::env;


// 导入处理文件相关事务
use std::fs;

fn main() {

    // 1. 接收命令行参数
    // 产生一个集合
    /*
        env::args() 无法处理非法的 unicode 字符，
        如果有非法的 unicode 字符，env::args() 就会 panic

        如果想处理非法的 unicode 字符，那么可以用 env::args_os()
        env::args_os() 返回 OsString
     */
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];
    println!("query = {:?}", query);
    println!("filename = {:?}", filename);

    // 2. 读取文件
    let contents = fs::read_to_string(filename).expect("read file failed");
    println!("file contents:\n{}", contents);


}
