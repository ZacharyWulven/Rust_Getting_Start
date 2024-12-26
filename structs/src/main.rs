

// fn main() {
//
//     test1();
//
//     test_tuple_sturct();
//
//     test_example();
//
//     let subject = AlwaysEqual;
// }

struct AlwaysEqual;


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

    println!("-----tnt------");
    println!("{rectangle:#?}");

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


// rust rover ------------------------------------------------

#[derive(Copy, Clone)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    /*
        &mut self 可变的
        如果想要调用下边方法，Rectangle2 创建完必须赋值给一个 mut 的变量
     */
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /*
        参数为 self，此方法会获取所有权
     */
    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Self {
            width: w,
            height: h
        }
    }

    fn set_to_max(&mut self, other: Self) {
        /*
            因为 max 方法是会获得所有权的，而 参数 &mut self 是没有所有权的, 所以报错
            修复方案：让 Rectangle2 实现 #[derive(Copy, Clone)] 后，max 方法就不再
                    需要所有权了
                    实际调用时是 Rectangle2::max(*self, other); 这样的
                    *self 解引用不会发生所有权的移动，而会发生复制，因为 Rectangle2 实现了
                    Copy, Clone Trait, 所以可以复制了

         */
        *self = self.max(other);
    }

}


struct NewPoint {
    x: i32,
    y: i32
}

fn print_point(p: &NewPoint) {
    println!("Point at ({}, {})", p.x, p.y);
}

// fn main() {
//     let mut p = NewPoint { x: 0, y: 0 };
//
//     let x = &mut p.x;
//
//     print_point(&p); // Error：这里需要 p 有读的权限，但 p 其实没有读权限
//
//     p.y = *x + 1;  // Ok：这样可以
//     *x += 1;
//     println!("{}", p.y);
// }

// 思考题1: 下边是否能通过编译
// fn main() {
//     let mut a = NewPoint { x: 1, y: 2 };
//     a.x += 1;
//     let mut b = NewPoint { x: 1, ..a };
//     a.x += 1;
//     println!("{}", b.x); // 1
// }

// 思考题2: 下边是否能通过编译
fn main() {
    let mut p = NewPoint { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;

    *x += 1;
    *y += 1;
    println!("{} {}", p.x, p.y); // 2, 3
}



