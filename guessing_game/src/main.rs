// 引用标准库 std 的 io 库 可以使用 io
/*
rust 默认把 prelude（预导入） 导入到程序的每个作用域中,如果你使用的类型 prelude 里没有
这时你需要自行导入, 就是使用 use 导入库
 */
use std::io;

// fn 即声明一个函数
fn main() {
    println!("猜数!");
    println!("猜一个数");

    let foo = 1;
    // foo = 2; // error cannot assign twice to immutable variable 

    let mut _bar = foo; // rust 默认变量都是 immutable 不可变的
    _bar = 3;  // 在变量前加 mut 表示变量是可变的 mutable

    // rust 中 String 使用 utf-8 编码
    // :: 两个冒号表示后边的是一个静态方法
    // 这里 new 函数创建一个空白的字符串
    let mut guess = String::new();

    // 1 读取一行东西，并放到 guess 字符串引用里
    // 2 expect 为抛出异常时候输出的错误信息
    // 3 & 表示参数是一个引用
    // 4 read_line 参数是需要一个引用
    // 5 引用默认也是不可变的比如 &guess, 所以 &mut 就是可变引用
    // 
    /* 
        6   
       read_line 返回 Result 类型，Result 就是枚举类型
       io::Result 有两个值 OK 和 Err
       如果是 OK 表示操作成功，并且里边有结果
       如果是 Err 表示操作失败，Err 里有失败 reason
       expect 是 Result 的方法，如果返回 Err 则 expect 会中断程序，将传入字符串显示
       如果返回 OK，expect 就将用户输入的值返回
     */
    io::stdin().read_line(&mut guess).expect("无法读取行");

    // {} 是一个占位，输出时候会被替换成 guess 变量的值
    // 一个 {} 对应一个变量的值
    println!("你猜测的数是：{}", guess);

}
