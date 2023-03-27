use std::error::{self, Error};
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

use std::net::IpAddr;


// Box<dyn Error> 是一个 trait 对象
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("error create file, {:?}", e),
    //         },
    //         // other_error 是自己起的名字
    //         other_error => panic!("error open file failed, {:#?}", other_error),
    //     },
    // };


    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|e| {
    //             panic!("error create file, {:?}", e)
    //         })
    //     } else {
    //         panic!("error open file failed, {:#?}", error);
    //     }
    // });


    // unwrap
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("open hello txt failed, {:#?}", error);
    //     }
    // };

    //let f = File::open("hello.txt").unwrap();

    //let f = File::open("hello.txt").expect("文件不存在");

    flood();

    test();
    let f = File::open("hello.txt")?;
    Ok(())
}

fn flood() {
    //let name = read_username_from_file();
    //let name = read_username_from_file_fast();
    let name = read_username_from_file_chain();

    match name {
        Ok(s) => println!("name = {}", s),
        Err(e) => println!("e is {}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 尝试打开文件，返回 Result<File, Error> 类型
    let f = File::open("hello.txt");

    // 对 Result<File, Error> 进行类型匹配
    let mut f = match f {
        Ok(file) => file, // 如果返回 File 将其赋值给 f
        Err(err) => return Err(err), // 否则方法返回 Error
    };

    let mut s = String::new();

    // 把文件内容读取到 s 字符串
    // 这个 match 表达式最后没有 ; ，即它是这个函数最后一个表达式，即函数结果
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // 如果成功将 s 字符串返回
        Err(err) => Err(err), // read_to_string 失败返回 err        
    }
}

fn read_username_from_file_fast() -> Result<String, io::Error> {

    let mut f = File::open("hello.txt")?;
    // 加问号等同于与下边 match
    // let mut f = match f {
    //     Ok(file) => file, // 如果返回 File 将其赋值给 f
    //     Err(err) => return Err(err), // 否则方法返回 Error
    // };

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn test() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);

}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value need be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn test_guess() {
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
    }
}