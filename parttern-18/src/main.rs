fn main() {

    //test_if_let();
    //test_while_let();
    //test_for();
    //test_let();
    //test_func_para();

    //test_liter();

    //test_name();

    //test_other();

    //test_point();

    //test_color();

    //test_p();

    //test__();

    //test_partion();

    //test_prefix();

    test_dot();
}

struct Position {
    x: i32,
    y: i32,
    z: i32,
}

fn test_dot() {
    let origin: Position = Position { x: 0, y: 0, z: 0 };
    match origin {
        // 匹配只需要 x 字段
        Position { x, .. } => println!("s is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 匹配只需要第一个和最后一个数
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    match numbers {
        // 这样会报错，因为不知道你要中间那个元素，会发生歧义
        // 使用 .. 时不能发生歧义
        (.., second, ..) => {
            println!("Some number: {}", second);
        }
    }


}

fn test_prefix() {
    // 未使用的变量以 _ 开头可以消除编译器的警告
    let _x = 5;
    let y = 10; 

    let s = Some(String::from("Hello"));

    // 写法 1
    // 这里 s 就 move 到 _s 了，使用 _s 会发生 bind 操作
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    //println!("{:?}", s); // 这里报错，因为 s 已经 move

    // 写法 2
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s); // 这里不会报错，因为 Some(_) 不会进行 bind 操作
}

fn test_partion() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // 匹配只要它俩都是 Some() 就可以，值不关心
        (Some(_), Some(_)) => {
            println!("Can not over write an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);




    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 匹配只需要知道第 1、3、5 个数，忽略其他数
        (first, _, third, _, fifth) => {
            println!("Some number: {}, {}, {}", first, third, fifth);
        }
    }

}


fn test__() {
    fn foo(_: i32, y: i32) {
        println!("this code only uses the y parameter: {}", y);
    }

    foo(3, 4);

}

fn test_p() {
    /*
        这里外层是一个元组，元组的第一个元素还是元组，第二个元素是结构体
     */
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32},     // 结构体的枚举变体
    Write(String),              // 元组的枚举变体，有一个元素
    ChangeColor(Color)
}

fn test_color() {

    let msg = Message2::ChangeColor(Color::HSV(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::RGB(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::HSV(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => (),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32},     // 结构体的枚举变体
    Write(String),              // 元组的枚举变体，有一个元素
    ChangeColor(i32, i32, i32), // 元组的枚举变体，有三个元素
}

fn test_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

}


struct Point {
    x: i32,
    y: i32,
}

fn test_point() {
    let p = Point { x: 0, y: 7 };
    
    // 这里使用模式对 p 进行结构
    let Point { x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("test point finished 1");

    // 简写版本
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    println!("test point finished 2");


    match p {
        // 匹配 x 是任何值，y 必须是 0
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", x),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

}

fn test_other() {
    let x = 5;
    match x {
        // 当 x >= 1 && x <=5 时走这个分支
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        // 当 x 是 a~j 时候就走这个分支
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn test_name() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn test_liter() {

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        //2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

}

fn foo(x: i32) {
    let a = Some(5);
    if let Some(x) = a {

    }

    if let x = 5 {
        
    }

}

/*
    参数时一个元组
    这里通过模式将元组的值提取给 x、y 变量里
    这样在函数内部就可以使用 x、y 了
 */
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn test_func_para() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn test_let() {
    let a = 5;
    // 这的模式是 x、y、z 分别对应 1、2、3 的值
    let (x, y, z) = (1, 2, 3);

    // 这样就会报错，因为变量与值的数对不上
    //let (x, y) = (1, 2, 3);
}

fn test_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using ur favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn test_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 只要 stack.pop() 返回的不是 None，这个循环就一直运行
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn test_for() {
    let v = vec!['a', 'b', 'c'];
    /*
        (index, value) 就是它的模式
        调用 iter().enumerate() 后每次返回一个元组
     */
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

