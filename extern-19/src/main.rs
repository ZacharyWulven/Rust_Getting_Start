/*
    extern "C" 块内的函数签名就是我们想调用的外部函数的签名
    而 "C" 指明了外部函数使用的是应用程序二进制接口（ABI），
    ABI 是用于定义在汇编层面的调用方式的
 */ 
extern "C" {
    fn abs(input:i32) -> i32;
}


// 这个函数在编译链接后就可以被 C 语言访问了
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just call a rust function from C!");
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

}
