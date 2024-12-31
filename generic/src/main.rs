use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");

    // test_origin();

    let str = String::from("Hello");
    let str2 = add_suffix(str);
    println!("str2 is {}", str2);


    // let s = String::from("Hello");
    // let s2;
    // let b = false;
    // if b {
    //     s2 = s;
    // }
    // println!("s {}", s);

}


fn add_suffix(mut s: String) -> String {
    s.push_str(", world!");
    s
}

fn test_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest2(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }


struct Point<T> {
    x: T,
    y: T,
}

// 泛型方法
impl<T> Point<T>  {
    fn getX(&self) -> &T {
        &self.x
    }
}

// 针对特定类型的方法
// 只有 Point<i32> 有 getX1 方法，其他 Point<T> 没有 getX1 方法
impl Point<i32>  {
    fn getX1(&self) -> &i32 {
        &self.x
    }
}

fn test_struct() {
    let p = Point { x: 1, y: 2};
    let p2 = Point { x: 1.0, y: 2.0};

}

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

struct Origin<T, U> {
    x: T,
    y: U,
}

impl<T, U> Origin<T, U> {
    fn mixup<V, W>(self, other: Origin<V, W>) -> Origin<T, W> {
        Origin { x: self.x, y: other.y }
    }
}

fn test_origin() {
    let p1 = Origin{ x: 1, y: 4};
    let p2 = Origin{ x: "hello", y: "world"};
    let p3 = p1.mixup(p2);
    println!("p3.x={}, p3.y={}", p3.x, p3.y);
}


/// ----- rust rover


// fix need where T: Debug
// fn print_slice<T>(v: &[T]) { // error code
fn print_slice<T>(v: &[T]) where T: Debug {
    for x in v {
        println!("{x:?}"); // error: `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
    }
}

impl Point<i32> {
    fn f(&self) -> &i32 {
        &self.x
    }
}

//
impl<T> Point<T> {
    fn f(&self) -> &i32 { // error：跟上边同名了
        &self.x
    }
}