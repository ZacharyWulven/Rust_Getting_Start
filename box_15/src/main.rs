use crate::List:: {Cons, Nil};

// 一个 tuple struct，即一个有名称的元组
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}



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

    let x = 5;
    //let y = &x;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    my_box_t();
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

fn my_box_t() {
    let x = 5;
    //let y = &x;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 这里报错，因为没实现 Deref trait


}