
enum List {
    //Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List:: { Cons, Nil };
use std::rc::Rc;

fn main() {
    // let a = Cons(5, 
    //     Box::new(Cons(10, Box::new(Nil))));
    // 这里获得了 a 的所有权
    // let b = Cons(3, Box::new(a));
    // 这里报错 因为 a 发生了 move，如果修改？把 Box 改成 Rc<T> 类型
    // let c = Cons(4, Box::new(a)); 


    let a = Rc::new(Cons(5, 
        Rc::new(Cons(10, Rc::new(Nil)))));  // a 的引用计数 1
    println!("Count after creat a = {}", Rc::strong_count(&a));

    // 这里不会获得 a 的所有权，而是 a 的引用计数 +1
    let b = Cons(3, Rc::clone(&a)); // a 的引用计数 2
    println!("Count after creat b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a)); // a 的引用计数 3
        println!("Count after creat c = {}", Rc::strong_count(&a));
        /*
            Rc::clone 不会深 copy，执行速度快
            而 a.clone() 会进行深 copy，需要花费大量时间来完成数据的 copy
        */
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));

}
