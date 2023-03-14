const MAX_HEIGHT: u32 = 100_000;
fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("the x is {}", x);

    x = 6;
    println!("the x is {}", x);

    println!("MAX_HEIGHT  is {}", MAX_HEIGHT);

    let y = 5;
    let y = y + 1;

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess {}", guess);
    
    let f = 2.0;

    let emoj = '🥰';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("{}, {}, {}, ", tup.0, tup.1, tup.2);
    println!("{}, {}, {}, ", x, y, z);

    let list = [1,2,3,4,5];
    let a: [i32;5] = [1,2,3,4,5];

    let a = [3; 5]; // 等于 [3,3,3,3,3]
    println!("{}", list[1]);

    another_function(5);

    test_loop();

    test_while();

    test_for();
}

fn another_function(x: i32) {
    println!("another function x is {}", x);

    let y = {
        x + 3  // x + 3 是这个块的返回值
        //x + 3;   // 语句默认返回 空 tuple 即 ()
    };
    println!("another function y is {}", y);

    println!("6 + 5 is {}", plus_five(6));

    let cond = true;
    let number = if cond { 4 } else { 2 };
    println!("number is {}", number);

}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn test_loop() {

    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("result is {}", result);
}

fn test_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }
    println!("while end");
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];
    // item is &i32
    for item in a.iter() {
        println!("the value is {}", item);
    }

    for num in (1..4).rev() {
        println!("the num is {}", num);
    }
}