use std::thread;
use std::time::Duration;
use core::cmp::PartialEq;

/*
    T 就是闭包的类型，使用 Fn trait
    它接收 u32 类型，返回一个 u32 类型
    闭包在运行前 value 的值是 None
    闭包运行后，会把值存放在 value 中
 */
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // 调用闭包 参数为 arg
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() { 
    
    let add_one_v2 = |x: u32| -> u32 {
        x + 1
    };

    let add_one_v3 = |x| {
        x + 1
    };
    // 由于函数体只有一行，因此 {} 可省略
    let add_one_v4 = |x| x + 1;

    let n3 = add_one_v3(3);
    let n4 = add_one_v4(3);

    // 在未使用闭包时，因为无法推断其类型，所以报错
    let example_closure = |x| x;
    // 一旦使用了，就能推断出 x 的类型了，就不报错了
    let s = example_closure(String::from("hello"));
    // 一旦类型确定了，就不能再改了, 这里就不能传 5 了
    // let s1 = example_closure(5);


    //generate_workout(10, 4);

    // capture();
    //
    // capture3();
    //
    // captureOne();

    capture_two();

    capture_three();
}

// '_ 表示返回的闭包依赖于某个生命周期
// fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {

    move || s_ref.to_string()
}

fn generate_workout(intensity: u32, random_number: u32) {
    /*
        定义闭包
        num 为参数
        将闭包赋值给 closure
     */
    // let closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn capture() {
    let x = 4;
    let equal_to_x = |z| z == x;
    assert!(equal_to_x(4));
    println!("{}", x);

    capture1();

}

fn capture1() {
    println!("capture1 begin");

    let x = vec![1, 2, 3];
    /*
        声明闭包，使用 move 关键字
        将 x 的所有权移动到了闭包里
     */
    let equal_to_x = move |z| z == x;
    // println!("can not use x here, because x did move: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn capture3() {
    println!("capture3 begin");

    let mut x = vec![1, 2, 3];
    let mut borrows_mutablly = || x.push(5);
    borrows_mutablly();
    println!("capture3 x is: {x:?}");

}

fn captureOne() {
    println!("captureOne begin");

    let mut x = vec![1, 2, 3];
    println!("Before defining closure: {x:?}");

    let only_borrows = || println!("From closure: {x:?}");
    println!("Before calling closure x is: {x:?}");
    only_borrows();
    println!("After calling closure x is: {x:?}");
}

fn capture_two() {
    println!("capture_two begin");

    let mut x = vec![1, 2, 3];
    println!("Before defining closure: {x:?}");

    let mut borrows_mutablely = || x.push(5);
    // println!("Before calling closure x is: {x:?}");
    borrows_mutablely();
    println!("After calling closure x is: {x:?}"); // [1, 2, 3, 5]
}

fn capture_three() {
    println!("capture_three begin");

    let mut x = vec![1, 2, 3];
    println!("Before defining closure: {x:?}");

    thread::spawn(move || println!("From thread: {x:?}"))
        .join()
        .unwrap();
}


#[cfg(test)]
mod tests {

    #[test]
    fn call_diff_value() {
        let mut c = super::Cacher::new(|x| x);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }
}