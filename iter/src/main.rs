use std::iter::Iterator;
use iter::Counter;

fn main() {
    one();

    let c = Counter::new();

    for i in c.into_iter() {
        println!("i={}", i)
    }

}

fn one() {
    let v1 = vec![1, 2, 3];
    // 这返回一个迭代器，没有使用所以没有任何效果
    let v1_iter = v1.iter();

    // 使用迭代器
    // for 取得了 v1_iter 的所有权，在其内部已经将其变为 mut
    for val in v1_iter {
        println!("Got: {}", val);
    }
    // 这里不能打印，因为已经没有了所以权
    //println!(": {:#?}", v1_iter);

}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn mp3() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len()  {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
