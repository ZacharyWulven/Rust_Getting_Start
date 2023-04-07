#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}



#[cfg(test)]

mod tests {
    use crate::{Shoe, shoe_in_my_size};


    #[test]
    fn iter_filter_my_shoes() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 12, style: String::from("sandal")},
            Shoe { size: 10, style: String::from("boot")},
        ];

        let my_shoes = shoe_in_my_size(shoes, 10);

        assert_eq!(my_shoes, vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")},
        ]);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        // 这里需要加 mut，因为调用 next 方法会改变迭代器里记录序列的变量
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iter_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let v2 = v1_iter.sum();
        assert_eq!(6, v2);
    }

    #[test]
    fn iter_sum_2() {
        let v1 = vec![1, 2, 3];

        // v1.iter().map(|x| x + 1) 并不会对每个元素进行加 1，因为没有消耗迭代器
        // v1.iter().map(|x| x + 1);

        // Vec<_> 由编译器自行推断
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

}