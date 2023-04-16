use std::ops::Deref;

// 一个 tuple struct，即一个有名称的元组
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // 定义了 Deref trait 的关联类型
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }

}

fn main() {
    println!("Hello, world!");

    let x = 5;
    //let y = &x;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    /*
        如果 MyBox 没实现 Deref trait，这里会报错
        *y 相当于 *(y.deref())，Rust 会隐式展开为 *(y.deref())
     */
    assert_eq!(5, *y);

    test_hello();

    test_drop();
}


fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn test_hello() {
    // 
    let m = MyBox::new(String::from("Rust"));

    /*
        &m 是 &MyBox<String> 类型
        由于 MyBox 实现了 deref trait，
        所以可以自动把 &MyBox 转化为 &String 
        String 也实现了 deref trait 会返回 &str
        最终 &m 的类型与 hello 函数的类型就匹配了
     */
    hello(&m);

    // 如果没有实现 deref trait 如何调用呢
    // 这样不便于阅读
    hello(&(*m)[..]);

    hello("Rust");

}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    /*
        drop 方法通常用于释放资源
        这里出于演示所以打印了一句话
     */
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` !", self.data);
    }
}



fn test_drop() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    drop(c);
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointer created.");
}