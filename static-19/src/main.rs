/*
    命名规范大写，声明时需要标识类型
    静态变量只能存储拥有 'static 这个生命周期的引用
    这就意味着 Rust 可以自己推断出其生命周期
    所以就无需手动标注了

 */
static HELLO_WORLD: &str = "Hello, world!";
fn main() {
    println!("name is {}", HELLO_WORLD);

    test_static();

}

//----------------------------------
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 访问和修改可变的静态变量是不安全（unsafe）的操作
    // 所以这里放到 unsafe 块里进行操作
    unsafe {
        COUNTER += inc;
    }
}

fn test_static() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

//----------------------------------
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
