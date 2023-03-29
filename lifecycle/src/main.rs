fn main() {
    println!("Hello, world!");

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    let str1 = String::from("abc");
    let str2 = "xyz";
    
    let l = large(str1.as_str(), str2);
    

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

