use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    let str1 = String::from("abc");
    {
        // str2 是静态的全程序作用域有效
        let str2 = "xyz";
        let result = large(str1.as_str(), str2);
        println!("{}", result);
    }

    // 编译有问题的
    // let str1 = String::from("abc");
    // let result;
    // {
    //     // str2 是静态的全程序作用域有效
    //     let str2 = String::from("xyz");
    //     // large 返回生命周期是 str1 与 str2 较短的那个
    //     // 所以这里用的是 str2 的生命周期, 赋给了 result 
    //     result = large(str1.as_str(), str2.as_str()); // error
    // } // 这里 str2 离开了作用域，但依然在借用的状态

    // // str2 必须在外部作用域结束前一直有效才行
    // // result 是 str2 的生命周期，str2 已经释放了 所以有问题
    // println!("{}", result);


    test_struct();

    last();
}

// 这个方法签名报错，因为没有说明 x 和 y 的生命周期
//fn large(x: &str, y: &str) -> &str {

// 加 'a 说明生命周期，表示 x、y、返回值的生命周期一样 
fn large<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn large2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn large3<'a>(x: &'a str, y: &str) -> &'a str {
//     let str = String::from("abc");
//     str.as_str() // 这是悬垂引用
// }

// 改成返回 String 就可以，这样相当于把 str 所有权返回给调用者
fn large3_3<'a>(x: &'a str, y: &str) -> String {
    let str = String::from("abc");
    str
}


struct Expert<'a> {
    part: &'a str, // 引用类型，即 part 这个引用要比 struct Expert 实例的生命周期要长
}

// impl<'a> 字段生命周期名字
// impl<'a> 与 Expert<'a> 不能忽略
impl<'a> Expert<'a> {
    // 根据规则一，就不用为 &self 添加生命周期了
    fn level(&self) -> i32 {
        3
    }
    // 根据规则一为参数 &self 和 announcement: &str 添加生命周期
    // 根据规则三 返回值被赋予了 self 的生命周期
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("announcement is {}", announcement);
        self.part
    }
}

fn test_struct() {
    let novel = String::from("call me jack. Some years ago ...");
    // &str
    let first = novel.split(".").next().expect("could not find a .");
    // 因为 first 生命周期 比 i 长，所以这样编译可通过
    let i = Expert { part: first };

    let s: &'static str = "I'm a static value";
}

fn longest_with_an_announcement<'a, T>
(x: &'a str, y: &'a str, ann: T) -> &'a str 
where T: Display {
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn last() {


}