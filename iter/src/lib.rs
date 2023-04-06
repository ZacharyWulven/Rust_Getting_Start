#[cfg(test)]

mod tests {

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        // 这里需要加 mut，因为调用 next 方法会改变迭代器里记录序列的变量
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

}