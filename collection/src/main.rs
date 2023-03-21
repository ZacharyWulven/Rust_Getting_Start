use collection::Vector;

fn main() {
    println!("Hello, world!");

    
    test_vec();
}

fn test_vec() {
    /*
        虽然 Rust 有类型推断
        但 new 返回空数组，无法推断类型，所以这里需要显示声明类型
        Vec 里可以存任意类型
     */ 
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v2 = vec![1,2,3];
    println!("{:?}", v2);

    let mut v3 = Vec::new();
    // 使用 push 添加元素
    v3.push(1); 


    let v4 = vec![1,2,3,4,5];
    let third = &v4[2];  // 使用索引方式访问，如果索引越界程序会 panic

    println!("The third element is {}", third);

    /*
        使用 get 方法访问
        get 方法参数是 索引，返回 Option<T>，所以可以使用 match
        如果索引越界，则 get 方法返回 None
     */
    match v4.get(100) { 
        Some(third) => println!("The third element is {}", third),
        None => println!("No third element"),
    }


    let mut v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0];  // 这里 first 是不可变的借用,引用其第一个元素
    //v5.push(6);   // 可以变借用, 这里报错, 因为已经借用为 first 不可变的了 
    println!("The first element is {}", first);
 
    
    // 遍历 vector
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        // *i 为解引用操作
        *i += 50;
    }

    println!("v6 {:?}", v6);



}