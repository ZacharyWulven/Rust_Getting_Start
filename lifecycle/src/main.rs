fn main() {
    println!("Hello, world!");

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    // let str1 = String::from("abc");
    // {
    //     // str2 是静态的全程序作用域有效
    //     let str2 = "xyz";
    //     let result = large(str1.as_str(), str2);
    //     println!("{}", result);
    // }

    let str1 = String::from("abc");
    let result;
    {
        // str2 是静态的全程序作用域有效
        let str2 = String::from("xyz");
        // large 返回生命周期是 str1 与 str2 较短的那个
        // 所以这里用的是 str2 的生命周期, 赋给了 result 
        result = large(str1.as_str(), str2.as_str()); // error
    } // 这里 str2 离开了作用域，但依然在借用的状态

    // str2 必须在外部作用域结束前一直有效才行
    // result 是 str2 的生命周期，str2 已经释放了 所以有问题
    println!("{}", result);

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

