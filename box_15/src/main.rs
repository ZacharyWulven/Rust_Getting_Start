use crate::List:: {Cons, Nil};
fn main() {
    println!("Hello, world!");
    /*
        5 存储在 heap 上
        b 走完 main 函数作用域会被释放
        会释放 stack 上的指针和存在 heap 上的数据
     */
    let b = Box::new(5);
    println!("b = {}", b);


    let list = Cons(1, 
        Box::new(Cons(2,
             Box::new(Cons(3,
                 Box::new(Nil))))));
    println!("{:#?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,  // 不需要空间
    Move {x: i32, y: i32}, // 需要两个 i32 空间
    Write(String),
    ChangeColor(i32, i32, i32),
}