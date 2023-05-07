use std::slice;

fn main() {
    fn_unsafe();
    test_address();
}

/*
    简单定义 split_at_mut 看看其实现的原理
 */
// fn split_at_mut_demo(slice: &mut [i32], mid: usize) 
// -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);
//     // 这里报错，因为违反了借用规则, 修正需要使用 unsafe code
//     (&mut slice[..mid], &mut slice[mid..])
// }

// 修正 split_at_mut_demo
/*
    使用 unsafe 代码修正 split_at_mut_demo
    split_at_mut 里使用了 unsafe 代码，但是其函数
    本身没有标记为 unsafe 所以 split_at_mut 就是一个
    unsafe 安全抽象，我们就可以从安全的代码中对其进行调用
 */
fn split_at_mut(slice: &mut [i32], mid: usize) 
-> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    /*
        这里返回一个原始指针，它的类型是 *mut i32
        它指向这个切片
     */
    let ptr = slice.as_mut_ptr();

    unsafe {
        /*
            from_raw_parts_mut 接收一个原始指针和一个长度来创建切片
            ptr.add(mid) 获得以 mid 为偏移量的原始指针
         */
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }

}

fn fn_unsafe() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    // r 是 v 的完整的 slice
    let r = &mut v[..];
    /*
        标准库提供 split_at_mut 函数，
        将参数 index 为界限，
        分隔成两个切片
        split_at_mut 实际是使用了不安全的代码
     */
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]); 
}

fn test_address() {
    println!("test_address");
    let address = 0x12345usize;
    let r = address as *mut i32;
    // 这样可能 crash 因为无法保证拥有 10000 元素的切片是有效的
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    }; 

}