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