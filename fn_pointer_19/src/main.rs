fn add_one(x: i32) -> i32 {
    x + 1
}
/*
    参数时 fn 即函数指针
    这个函数指针要求参数时 i32，返回值也是 i32
 */
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_fn() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn test_fn2() {

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    /*
        这里使用闭包
        闭包的参数时 i，然后调用 i 的 to_string() 方法
     */
    .map(|i| i.to_string())
    .collect();
    println!("list_of_strings is {:#?}", list_of_strings);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    /*
        这里将 ToString::to_string 函数传入
     */
    .map(ToString::to_string)
    .collect();

    println!("list_of_strings is {:?}", list_of_strings);

    /*
        map 的参数需要实现 FnMut Trait 
        而闭包和函数指针 ToString::to_string 都实现了 FnMut Trait
     */
}

fn test_fn3() {
    enum Status {
        Value(u32),
        Stop,
    }

    /*
        Value(3) 与函数调用相似
        实际上这种构造器确实被实现成了函数
        这个函数会接收一个参数并返回新的实例

        所以我们可以把这种构造器也作为实现了闭包 Trait 的函数指针来使用

     */
    let v = Status::Value(3);


    let list_of_statuses: Vec<Status> = 
    (0u32..20)
    // 这里就是把构造器直接传入即可
    .map(Status::Value)
    .collect();
}

//------------------------------
// 这样想返回一个闭包时不可以的 
/*
    这样想返回一个闭包时不可以的，编译报错
    因为在编译时无法获取其已知的大小
 */
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// 解决方案
// 即将其放在某种指针的后边
/*
    解决方案
    即将其放在某种指针的后边
    这时它的返回类型就有固定的大小了，编译就不会出现问题了
 */
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn main() {
    test_fn();
    test_fn2();

    test_fn3();

}


