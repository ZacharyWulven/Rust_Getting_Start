use std::fmt;

// 类型别名 1
type Kilometers = i32;


// 类型别名 2
type Thunk = Box<dyn Fn() + Send + 'static>;

fn take_long_type(f: Thunk) {

}

fn return_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

// 类型别名 3

// 由于这个声明在 use std::io::Error 中所以这里需要注释
//type Result<T> = Result<T, std::io::Error>;

// 可以再别名一次，以便直接使用 Result<T>
type Result<T> = std::io::Result<T>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;

}



fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    //-----------
    let f: Thunk = Box::new(|| println!("hi"));
}
