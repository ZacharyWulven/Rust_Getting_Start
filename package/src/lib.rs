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

}

pub fn eat_at_restaurant() {

    // 使用绝对路径调用
    // lib.rs 隐式组成了名为 crate 的模块
    // 因为 eat_at_restaurant 与 front_of_house 都在 crate 根级，所以不需要声明 pub 就可相互调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
    front_of_house::hosting::add_to_waitlist();

}