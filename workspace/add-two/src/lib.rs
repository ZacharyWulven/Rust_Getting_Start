pub fn add_two(left: i32) -> i32 {
    left + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
