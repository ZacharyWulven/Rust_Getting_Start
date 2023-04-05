/*
    用于读取命令行参数，由标准库提供
    args 返回一个迭代器，迭代器返回一系列值，调用 collect 可将其转化为一个集合
    std::env::args()
 */
// use std::env;

// // 导入处理文件相关事务
// use std::fs;
// use std::process;
// use std:: error::Error;

use  std::{ env, fs, process, error::Error };



pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    // 参数为 vec 的切片
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // 使用 clone() 将 &str 转为 String
        let query = args[1].clone();
        let filename = args[2].clone();
        // 
        // 使用 std::env 模块
        // 只要 CASE_INSENSITIVE 变量出现 就是不区分大小写的 
        /*
            insensitive 取自环境变量，使用 std::env 模块 
            只要 CASE_INSENSITIVE 变量出现 就是不区分大小写的
            var 返回 Result 对象
            如果设置了环境变量 is_ok 返回 true
            没设置环境变量就是区分大小写的
         */
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
        println!("case_insensitive: {}", case_insensitive);
        Ok(Config { query , filename, case_insensitive })
    }
}


// Box<dyn Error> 理解为实现了 Error 这个 trait 的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 2. 读取文件
    //let contents = fs::read_to_string(config.filename).expect("read file failed");
    
    // ? 不会发生 panic，如果发生错误会把错误 return 给调用者
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_insensitive {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("line: {}", line);
    }

     // 如果没有问题，我们返回 Ok
    Ok(())
}

/*
    返回值是从 contents 里取的，所以它俩要有相同的生命周期
 */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // lines() 返回一个迭代器
   for line in contents.lines() {
       if line.contains(query) {
            results.push(line);
       }
   }
   results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase(); 
    // lines() 返回一个迭代器
   for line in contents.lines() {
       if line.to_lowercase().contains(&query) {
            results.push(line);
       }
   }
   results
}


// 添加测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents));
    }

}
