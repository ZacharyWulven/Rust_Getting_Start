/*
    用于读取命令行参数，由标准库提供
    args 返回一个迭代器，迭代器返回一系列值，调用 collect 可将其转化为一个集合
    std::env::args()
 */
use std::env;


// 导入处理文件相关事务
use std::fs;
use std::process;

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

    // let query = &args[1];
    // let filename = &args[2];

    /*
        如果 new 返回的是 Ok，unwrap_or_else 会将 Ok 的值取出并返回
        如果 new 返回的是 Err，就会调用一个闭包(匿名函数)
     */
    let config = Config::new(&args).unwrap_or_else(|err| {
        // |err| 是闭包的参数
        println!("Problem parsing arguments: {}", err);
        /*
            调用 exit，程序会立即终止
            参数 1 即状态码
            可以使用 cargo run 试试
         */
        process::exit(1);
    });

    println!("query = {:?}", config.query);
    println!("filename = {:?}", config.filename);

    // 2. 读取文件
    let contents = fs::read_to_string(config.filename).expect("read file failed");
    println!("file contents:\n{}", contents);


}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 参数为 vec 的切片
    fn new(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // 使用 clone() 将 &str 转为 String
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query , filename })
    }
}


