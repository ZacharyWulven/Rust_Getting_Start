// 引入时直接指定到 HashMap 本身
use std::collections::HashMap;

use std::fmt::Result;
// use std::io::Result as IOResult;  // 设置本地别名为 IOResult

//use std:: { collections::HashMap, fmt::Result, io::Result as IOResult};

use rand::Rng;

use std::io:: {self, Write};

use std::collections::*;

// fn f1() -> Result {

// }

// fn f2() -> IOResult<i32> {

// }

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);    
}
