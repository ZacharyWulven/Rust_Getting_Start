fn main() {
    let mut num = 5;

    /*
        在 unsafe 代码块之外创建原始指针
        但只能在 unsafe 代码块中对原始指针解引用
    */
    // 将不可变引用转为不可变原始指针，这是一个有效的原始指针
    let r1 = &num as *const i32;

    // 将可变引用转为可变原始指针，这是一个有效的原始指针
    let r2 = &mut num as *mut i32;

    // 对原始指针解引用
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);   

        // 可以用 r2 修改其值，这需要多加小心
        *r2 = 3; 
        println!("r2: change to {}", *r2);   
    }

    /*
        创建一个无法知道其有效性的原始指针
        原始指针不一定一直有效

        这个内存地址可能有数据，也可能没有数据，这样依然可以创建一个原始指针
        address 可能是无效的
     */
    let address = 0x1245usize;
    let r = address as *const i32;
    // 对原始指针解引用
    unsafe {
        println!("r: {}", *r);
    }

}
