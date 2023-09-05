fn serve_common_order() {}


mod big_house;

mod front_of_house {

    pub mod hosting { // 这是一个子 module
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    fn serve_order() {
        // super 找到 front_of_house 的上级模块，调用 serve_common_order
        super::serve_common_order(); 

    }


}

pub fn eat_at_restaurant() {

    // 使用绝对路径调用
    // lib.rs 隐式组成了名为 crate 的模块
    // 因为 eat_at_restaurant 与 front_of_house 都在 crate 根级，所以不需要声明 pub 就可相互调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    // 一个 public 的 struct
    pub struct Breakfast {
        pub toast: String,      // public
        seasonal_fruit: String, // private
    }

    impl Breakfast {
        // 关联函数
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"),
             }
        }
    }
}

pub fn eat_up() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //meal.seasonal_fruit = String::from("bluecerries"); // 这里报错，因为 seasonal_fruit 是私有的
}

// 引入模块 hosting 
//use crate::front_of_house::hosting;

// 使用相对路径引入 hosting，因为 front_of_house 在同一级
//use front_of_house::hosting;

use big_house::host::host;

/*
    使用 use 将路径（名称）导入到作用域内，该名称在此作用域内是私有的
    可以使用 pub use 将其声明为 public，这样外部就可以调用了
 */
pub use crate::front_of_house::hosting;

pub fn eat_use() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // hosting::seat_at_table(); 报错 因为这个方法是私有的
}

pub fn big_house() {
    host::check_host();
}