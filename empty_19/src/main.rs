// fn bar() -> ! {
// }

// 例子 2 此代码报错，只用于理解 Never
// impl <T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             // 这个分支返回 T 类型
//             Some(val) => val,
//             /*
//                 而 panic! 的返回类型就是 Never
//                 panic 时会终止程序，不会返回任何值
//                 所以 None 时，不会返回值
//                 所以 unwrap 的代码就是合理的
//              */
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

fn generic<T>(t: T) {

}
// 上边会被隐式转化为下边这样
fn generic<T: Sized>(t: T) {

}

fn generic<T: ?Sized>(t: &T) {

}


fn main() {
    let guess = "";

    // loop {
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         /*
    //             这里 continue 就是 Never 类型
    //             而 Never 类型无法产生一个可供返回的类型
    //             所以这个 match 返回的类型就采用了 Ok 分支的 u32 类型
    //             Never 的表达式就被强制转成其他的任意类型了
    //             所以 match 的两个分支的返回类型就是一样的类型了
    //          */
    //         Err(_) => continue,
    //     };
    // };


    // loop {
    //     /*
    //         这个循环永远不会结束
    //         它的返回类型就是 !
    //         当然可以使用 break 指令终止循环
    //      */
    //     print!("and ever ");
    // }

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    
}
