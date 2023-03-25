use std::error;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
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

    let f = File::open("hello.txt").expect("文件不存在");

}
