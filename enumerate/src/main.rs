fn main() {

    /*
        创建一个枚举值
        枚举的变体（枚举值）都位于标识符的命名空间下，使用两个冒号进行分隔
     */
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    println!("{:#?}", four);


    let local = IPAddress {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:#?}", local);

    let loopback = IPAddress {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?}", loopback);

    let home = IPAddr1::V4(127, 0, 0, 1);
    let lb = IPAddr1::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", lb);

    test_message();

    test_options();

    test_match();

}

// 
fn route(ip: IPAddrKind) {

}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IPAddr1 {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
struct IPAddress {
    kind: IPAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32}, // 关联的数据类型是一个匿名的 struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) { 
        println!("Hello");
    }
}

fn test_message() {

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 12 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
    
    q.call();
}

fn test_options() {

    let some_str = Some("A String");
    let some_num =  Some(5);

    let absent_num: Option<i32> = None; // 这是一个无效的值

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y; // 这里报错，因为 y 不是 i8 类型


}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    /*
        match 会把 coin 与里边模式依次进行比较
        如果某匹配成功，那个模式的代码就被执行，模式后边的是一个表达式
        如果匹配失败，就往下继续匹配
        匹配成功代码表达式会作为整个 match 的最终结果返回

        这个函数的最后一个表达式就是 match 表达式
     */
    match coin {
        Coin::Penny => 1, // 这里 Coin::Penny 是一个模式, 简单表达式 => 直接写就行
        Coin::Nickel => { // 复杂表达式需要 => 后使用 {}
            println!("5");
            5
        },
        Coin::Dime => 10,  
        Coin::Quarter(state) => { //绑定值的模式匹配
            println!("State quarter from {:#?}!", state);
            25
        },
    }
}


fn test_match() {

    let cent = value_in_cents(Coin::Quarter(USState::Alabama));
    println!("{}", cent);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let v = 0u8; // 定义一个 u8 类型
    match v {
        1 => println!("one"),
        2 => println!("two"),
        other => println!("other {:#?}", other),
        // _ => (), // 使用 _ 通配符代替，_ 通配符必须在最后一行写。它的代码是()一个空元组，即什么也不做
    }



    let v2 = Some(3u8);
    match v2 {
        Some(3) => println!("three"),
        _ => println!("if let else others"),
    }

    if let Some(3) = v2 {
        println!("if let three");
    } else {
        println!("if let else others");
    }


}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}