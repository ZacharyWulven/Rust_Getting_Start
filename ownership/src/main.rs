fn main() {

    // let mut s1 = String::from("hello");
    // //s.push_str(", World!");

    // //let s2 = s1;
    // println!("{}", s1); // error, value borrowed here after move

    // let s2 = s1.clone();
    // println!("s1 is {}, s2 is {}", s1, s2);

    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);

    let s = String::from("Hello World");

    // 这里 s 已经 move 到函数中了
    take_ownership(s);

    //println!("s is {}", s); // 由于 take_ownership 调用 s 已经 Move 了，所以 s 不再可用

    let x = 5;

    makes_copy(x);

    println!("x is {}", x);

    // 将 str 的 hello 移动给了 s1
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // 这里其实是 s2 Move 给了 s3
    let s3 = takes_and_gives_back(s2);

    // 作用域结束 s1、s3 drop，s2 因为已经 Move 了，所以不会有什么变化
}

fn take_ownership(string: String) {
    println!("string is {}", string); // 传进来的 s 在这里可用，在函数结束 drop
}

fn makes_copy(number: i32) {
    println!("num is {}", number);
}


fn gives_ownership() -> String {
    let str = String::from("hello");
    str
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}