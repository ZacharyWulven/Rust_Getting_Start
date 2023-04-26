fn main() {

    //test_if_let();
    //test_while_let();
    //test_for();
    //test_let();
    test_func_para();
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

