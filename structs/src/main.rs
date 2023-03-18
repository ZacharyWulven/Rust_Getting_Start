

fn main() {

    test1();

    test_tuple_sturct();

    test_example();
}


struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

fn test1() {

    let mut user = User {
        email: String::from("tom@gmail.com"),
        sign_in_count: 42,
        name: String::from("tom"),
        active: true,
    };
    println!("{}", user.email);

    user.email = String::from("zack@gmail.com");

    println!("{}", user.email);

    let u1 = easy_build_user(String::from("jack"), String::from("jack@gmail.com"));
    println!("{}", u1.email);

    let u2 = User {
        email: String::from("u2@gmail.com"),
        name: String::from("u2"),
        ..u1
    };
    println!("{}", u2.email);



}

fn build_user(name: String, email: String) -> User {
    User {
        email: email,
        sign_in_count: 42,
        name: name,
        active: true,
    }
}

// 字段初始化简写
fn easy_build_user(name: String, email: String) -> User {
    User {
        email,
        sign_in_count: 42,
        name,
        active: true,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test_tuple_sturct() {


    let black = Color(0, 0, 0);

    let origin = Point(0,0,0);

    // black 与 origin 是不同类型，因为是不同的 tuple struct 的实例
    println!("{}", black.0);

}

fn test_example() {
    let w = 30;
    let l = 50;
    println!("area = {}", area(w, l));

    let rect = (30, 50);
    println!("area = {}", area_tuple(rect));

    let rectangle = Rectangle{width: 30,  length: 50};
    println!("area = {}", area_struct(&rectangle));

    // 
    /*
        {} 告诉 println! 使用 Display trait 格式化
        而有些类型没有实现 Display trait 就会报错 
        如果修复
        方式一：可以使用 {:?}, 但需要在 struct 定义上一行声明 #[derive(Debug)]
        #[derive(Debug)] 是让 struct 选择 Debug 功能
        方式二：使用 {:#?} 
         
     */
    println!("{:#?}", rectangle);

    println!("rectangle.area {}", rectangle.area());

    // 调用关联函数
    println!("{:#?}", Rectangle::square(30));


}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// derive 是派生的意思
#[derive(Debug)] // 意思是让 Rectangle 派生于 Debug 这个 trait
struct Rectangle {
    width: u32,
    length: u32, 
}

// 为 struct 定义方法需要 impl 块
impl  Rectangle  {
    // self 就是 Rectangle 类型
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

}

impl Rectangle {
    // 这是一个关联函数
    fn square(width: u32) -> Rectangle {
        Rectangle { width, length: width }
    }

}

// 传递引用，不获取其所有权
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}