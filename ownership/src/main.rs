use std::mem::forget;
use std::ptr::addr_of_mut;
use std::rc::Rc;

/*
fn main() {

    //test_ownership();

    // test_reference();

    // mut_refer();

    // bu_chong()

    // move_sample()

    // box_sample()

    // down()

    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let full = stringify_name_with_title(&name);
    println!("{}", first);
    println!("{}", full);
    println!("{:?}", name);

}
*/

// 修复所有权常见的错误 Begin
// 1 Fixing an Unsafe Program: Returning a Reference to the Stack

// 方案一
// fn return_a_string() -> &String {
//     let s = String::from("hello");
//     &s
// }


// 方案二
// &'static 会随程序运行一直存在，活的最长，除非程序嘎了
// fn return_a_string() -> &'static str {
//     "Hello, world!"
// }

// 方案三
// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("hello"));
//     Rc::clone(&s)
// }

// 方案四
fn return_a_string(output: &mut String) {
    // 将整个字符串替换为 "Hello World"
    output.replace_range(.., "Hello World");
}

// 2 Fixing an Unsafe Program: Not Enough Permissions

// Question
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));  // 没有写的权限，所以报错
//     let full = name.join(" ");
//     full
// }

// 方案一
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("Esq."));  // 没有写的权限，所以报错
//     let full = name_clone.join(" ");
//     full
// }


// 方案二
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" "); // join 返回一个新 String
    full.push_str("Esq.");
    full
}


// 3 Fixing an Unsafe Program: Aliasing and Mutating a Data Structure (同时启用别名和可变性)
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     // 本来 dst 是可变引用，由于下边行走完就有了一个引用指向 dst 里的元素，导致 dst 失去了写的权限
//     // ，失去了可变性
//     let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
//
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// 方案一
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     // 本来 dst 是可变引用，由于下边行走完就有了一个引用指向 dst 里的元素，导致 dst 失去了写的权限
//     // ，失去了可变性
//     let largest = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
//
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// 方案二: 创建新的 Vec 弊端是涉及大量内存消耗
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     // 本来 dst 是可变引用，由于下边行走完就有了一个引用指向 dst 里的元素，导致 dst 失去了写的权限
//     // ，失去了可变性
//     let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
//
//     let to_add = src.iter()
//         .filter(|s| s.len() > largest.len())
//         .cloned().collect();
//
//     dst.push(to_add);
//
//
// }

// 方案三: 创建新的 Vec，相较于方案一更浪费内存
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    // 本来 dst 是可变引用，由于下边行走完就有了一个引用指向 dst 里的元素，导致 dst 失去了写的权限
    // ，失去了可变性
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }


}

// 4 Fixing an Unsafe Program: Copying vs. Moving Out of a Collection

// Question
// fn main() {
//     // Copy Trait
//     let v: Vec<i32> = vec![0, 1, 2];
//     let n_ref: &i32= &v[0];
//     let n: i32 = *n_ref; // 由于是 i32 类型，这里会发生 Copy
//
//     // Move
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let n_ref: &String= &v[0];
//     /*
//         下边 error: 无法移动, 引用是没有所有权的, 想通过解引用获得所有权是不行的
//         下边发生的实际是想要获得所有权，而引用是没有所有权的
//      */
//     let n: String = *n_ref;
// }

/*
    * 如果一个值不拥有堆数据，那么它可以在不移动的情况下被复制
    - 一个 `i32` 不拥有堆数据，因此可以在不移动的情况下被复制
    - 一个 `String` 拥有堆数据，因此`不能`在不移动的情况下被复制
    - 一个 `&String` 不拥有堆数据，因此可以在不移动的情况下被复制

 */

// fn main() {
//     // Copy Trait
//     let v: Vec<i32> = vec![0, 1, 2];
//     let n_ref: &i32= &v[0];
//     let n: i32 = *n_ref; // 由于是 i32 类型，这里会发生 Copy
//
//     // Move
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let n_ref: String= v[0].clone();
//
// }

// 5 Fixing an Unsafe Program: Mutating Different Tuple Fields（修改 Tuple 不同字段）

// 没有问题的 case

// fn main() {
//     let mut name = (String::from("Ferris"), String::from("Rustacean"));
//     // 下边这句走完 name 对第一个元素就不具备写权限了，同时 name 失去了对整个 tuple 的写权限
//     let first = &name.0;
//     name.1.push_str(", Esq."); // 但 name.1 依然有写权限
//     println!("{first}, {}", name.1);
// }

// 有问题的 case
// fn main() {
//     let mut name = (String::from("Ferris"), String::from("Rustacean"));
//     let first = get_first(&name);
//     // 报错：rust 只看方法签名，get_first 函数返回 &String，所以 rust 认为 name.0 和 name.1 都被不可变借用
//     name.1.push_str(", Esq.");
//     println!("{first}, {}", name.1);
// }
//
// //
// fn get_first(name: &(String, String)) -> &String {
//     &name.0
// }



// 6 Fixing an Unsafe Program: Mutating Different Array Elements
// fn main() {
//     let mut a = [0, 1, 2, 3];
//
//     let y = a[2];
//
//     let x = &mut a[1];  // 可变借用，这行走完，a 失去所有权限，只有 x 有权限
//
//     // let y = &a[2];  // 不可变借用，由于 a 没有任何权限了，所以这里也不可能给予读权限，所以报错
//     *x += y;             // 可变借用持续到这行结束，所以在此之前不能有不可变借用
//
//     println!("{a:?}");
// }






// 修复所有权常见的错误 End


fn down() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2 = &*num;
    println!("*num is {}, *num2 is {}", *num, *num2);
}

fn sa() {
    let x = 0;
    let mut x_ref = &x;
    let y = 1;

    // *x_ref += 1; // err: 因为其借用的是不可变的
    x_ref = &y;  // Ok: 因为 x_ref 是可变的可以指向其他 &i32 地址
}

fn box_sample() {

    let x = Box::new(5);
    let y = x;
    // println!("{}", x); // 已经 move，不能让多个 `Box` 同时拥有一块数据
    println!("{}", y);

    let r1 = &y;  // 多个引用可以同时指向同一个 Box
    let r2 = &y;
    println!("r1: {r1}, r2: {r2}");

    let r3 = &*y;
}

fn bu_chong() {
    let m1 = String::from("hello");
    let m2 = String::from("world");
    println!("m1:{}", m1);
    println!("m1 的地址:{:p}", &m1);
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
}

fn move_sample() {
    // compile error sample
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // v.push(4);
    println!("num is {:?}", *num);
}

fn greet(g1: &String, g2: &String) {
    println!("g1:{}, g2:{}!", g1, g2);
    let address_in_g1 = g1 as *const String;
    println!("g1 value:{g1}");
    println!("g1 存的内容:{:p}", address_in_g1);
    println!("g1的地址:{:p}", &g1);
}

fn test_ownership() {

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


fn test_reference() {
    let mut s1 = String::from("hello");

    /*
        此处只是创建一个 s1 的引用传入了参数，并不拥有 s1，并没有转移所有权
        所以当这个引入走出作用域，并不会把 s1 清理掉
     */
    let len = get_length(&mut s1);

    println!("s1 = {}, len = {} ", s1, len);
}

fn get_length(str: &mut String) -> usize {
    /*
    当一个函数的参数是引用时，函数作用域结束，不需要考虑所有权，因为引用没有获得数据的所有权
     */

    str.push_str("world"); // 这里报错，不能修改，因为引用时不可变的
    str.len() // 这里因为是引用，没有所有权，所以走出作用域什么不会做
}

fn mut_refer() {
    let mut s = String::from("hello");

    let s1 = &mut s;
    //let s2 = &mut s; // 这里报错，因为这个作用域 s 可变引用个数不能超过一个

    //println!("s1 {}, {}", s1, s2);

    mut_refer2();

    mut_refer3();

    //dangle()

    test_slice();
}

fn mut_refer2() {
    let mut s = String::from("hello");
    {
        let s1 = &mut s;
    }

    let s2 = &mut s;

    println!("s1 {} ", s2);
}

fn mut_refer3() {
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; // 这里报错，因为 s 因为借用为不可变的引用了

    // println!("s1 {}, s2 {}, s3 {}", s1, s2, s3);
}

fn dangle() {
    //let s = _dangle();
}

// 这个函数会返回一个悬空指针，但 Rust 中编译就会报错
// fn _dangle() -> &String {
//     let mut s = String::from("hello");
//     &s
// }

fn test_slice() {

    let mut s = String::from("Hello World");
    let wordIdx = first_word(&s);

    // 范围 [start, end)
    //let hello = &s[0..5];
    let hello = &s[..5];

    // let world = &s[6..11];
    let world = &s[6..];

    let whole = &s[..];

    // s.clear(); 如果这里将字符串清空，wordIdx 就没有用了
    println!("space index is {}", wordIdx);

    println!("whole is {}", whole);

    let slice = first_word2(&s);
    println!("slice is {}", slice);

    let s1 = "hello";

    let w = first_word3(&s[..]);
    let w1 = first_word3(s1);
    println!("w is {}, w1 is {}", w, w1);


    let a = [1, 2, 3, 4, 5];
    let sliceA = &a[1..3];



}

fn first_word(s: &String) -> usize {
    // 得到一个字节数组, 类型是 &[u8]
    let bytes = s.as_bytes();

    /*
        iter() 为其创建一个迭代器, 这个方法依次返回集合中的每个元素
        enumerate() 会把 iter() 返回的结果进行包装，并把每个结果作为元组的一部分进行返回
        i 就是索引
        &item 就是元素，这里就是字符串里的字节，注意 这里它是一个引用
     */
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' 就是空格字节
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// &str 表示字符串切片类型
fn first_word2(s: &String) -> &str {
    // 得到一个字节数组, 类型是 &[u8]
    let bytes = s.as_bytes();

    /*
        iter() 为其创建一个迭代器, 这个方法依次返回集合中的每个元素
        enumerate() 会把 iter() 返回的结果进行包装，并把每个结果作为元组的一部分进行返回
        i 就是索引
        &item 就是元素，这里就是字符串里的字节，注意 这里它是一个引用
     */
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' 就是空格字节
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

// &str 表示字符串切片类型
fn first_word3(s: &str) -> &str {
    // 得到一个字节数组, 类型是 &[u8]
    let bytes = s.as_bytes();

    /*
        iter() 为其创建一个迭代器, 这个方法依次返回集合中的每个元素
        enumerate() 会把 iter() 返回的结果进行包装，并把每个结果作为元组的一部分进行返回
        i 就是索引
        &item 就是元素，这里就是字符串里的字节，注意 这里它是一个引用
     */
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' 就是空格字节
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("hello");
    for &item in s.as_bytes().iter() {
        if item == b'l' {
            s.push_str(" world");
        }
    }
    println!("{}", s);
}