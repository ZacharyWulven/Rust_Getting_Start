use crate::models::structs::HousePrice;
use crate::models::enums::YesNo;

fn main() {
    println!("Hello, world!");
    let y = crate::m1::m2::method_1();

    let house_price = HousePrice {
        price: 1000,
        area: String::from("Center"),
        bed_rooms: 6,
        main_road: YesNo::No
    };

    // read_csv("ss")
}

/*
    rust 会从三个地方找这个 modules 模块
    1. inline 模式：声明模块的地方是否有大括号
    2. 声明到 modules.rs 里
    3. 找 models/mod.rs 文件
 */
mod models;

mod m1 {
    pub mod m2 {
         pub fn method_1() {
             println!("Method 1");
         }
    }
}

mod x1 {
    mod x2 {
        use crate::x1::x2;

        fn method_2() {
            super::super::m1::m2::method_1();
            println!("call m1::m2::Method 1");
        }

        fn method_3() {
            // x2::method_2();
            self::x2::method_2();
        }
    }
}