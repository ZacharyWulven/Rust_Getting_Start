use std::fmt;

/*
    我们想为 Vec 实现 Display 这个 trait
    而 Vec 和 Display 都定义在外部的包中，
    所以我们无法直接为 Vec 实现 Display 这个 trait

    那么怎么做呢？
    我们可以把 Vec 包裹在 struct Wrapper 中
    

 */
// 包裹 Vec 以便实现 Display trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
            因为 Wrapper 是一个 tuple struct 可以通过索引将 Vec 取出
         */
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
}
