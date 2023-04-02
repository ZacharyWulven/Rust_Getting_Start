pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    internal_add(a, 2)
}

// private func
fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

fn greeting1(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);

        } else if value > 100 {
             
        }
        if value < 1 || value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // 将外部模块所有内容都导入到这个模块
    use super::*;

    #[test]
    fn it_work_internal() {
        assert_eq!(4, internal_add(2, 2));
    }

    #[test] // 这是一个测试函数
    fn explorer() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn make_panic() {
        //panic!("make me panic");
    }

    #[test]
    fn another() {
        let a = true;
        assert!(a);
    }

    #[test]
    fn it_work_two() {
        //assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_work_greeting() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn it_work_greeting1() {
        let result = greeting1("Carol");
        assert!(
            !result.contains("Carol"), 
            "Greeting did not cntain name {}", result
        );
    }

    #[test]
    //#[should_panic]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]

    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_work_result() -> Result<(), String>{
        if 2 + 2 == 4 {
            println!("2 + 2 is 4");
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn it_work_igonre() {
        assert_eq!(2, 1 + 1);
    }
    

}
