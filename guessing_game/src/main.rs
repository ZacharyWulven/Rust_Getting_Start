// 引用标准库 std 的 io 库 可以使用 io
/*
rust 默认把 prelude（预导入） 导入到程序的每个作用域中,如果你使用的类型 prelude 里没有
这时你需要自行导入, 就是使用 use 导入库
 */
use std::io;
// Rng 是一个 trait，可以看成是一个协议
use rand::Rng; // trait

// 引入 Ordering，Ordering 是一个枚举类型它有三个值
use std::cmp::Ordering;

// fn 即声明一个函数
fn main() {
    println!("猜数!");

    let foo = 1;
    // foo = 2; // error cannot assign twice to immutable variable

    let mut _bar = foo; // rust 默认变量都是 immutable 不可变的
    _bar = 3; // 在变量前加 mut 表示变量是可变的 mutable

    /*
        ThreadRng 就是一个随机数生成器，它是位于本地线程空间
        并通过系统获得随机数的种子
        gen_range 在 [1, 101) 之间获取随机数
    */
    let rand_number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字是 {}", rand_number);

    // 无限循环使用 loop 关键字
    loop {
        println!("猜一个数---- Begin");
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

        // 类型隐藏，rust 可以让定义同名的变量，一般用于类型转换
        // 从这行后边 guess 就是 u32 类型了
        // trim 会去掉字符串的空白
        // parse 将字符串转为 number 类型
        // shadow
        //let guess: u32 = guess.trim().parse().expect("Please type a number !");

        // 优化 match 是 rust 进行匹配的重要模式
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // if 是 ok 则将值返回
            Err(_) => continue, //  if 是 error 则跳过本次进行下一次循环
        };

        // {} 是一个占位，输出时候会被替换成 guess 变量的值
        // 一个 {} 对应一个变量的值
        println!("你猜测的数是：{}", guess);

        // 拿 guess 与 rand_number 进行比较
        /*
           Ordering 是一个枚举类型它有三个值，Less、Greater、Equal


           一个 match 表达式由多个 arm 组成，
           如果 match 后边的值跟某个 arm 匹配了，那就执行那个 arm 的语句，
           match 匹配是从上到下代码进行匹配的
        */
        /*
           rust 是静态的强类型语言，有类型推断的能力
           把 match 下注释掉 rand_number 就变成 i32 类型了，
           有下边 match 代码 rand_number 就是 u32 类型，因为 guess 是 u32，比较时类型要一样
        */
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small"), // 这是一个 arm
            Ordering::Greater => println!("Too great"),
            Ordering::Equal => {
                println!("You win！");
                break; // 跳出 loop 循环
            }
        }
        println!("猜一个数---- End");
    }
}
