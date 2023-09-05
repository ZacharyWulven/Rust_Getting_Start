use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    test_vec();

    example();

    test_string();

    test_hash_map();
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


    let v7 = vec![1, 2, 3, 4];
    for i in &v7 {
        println!("v7 item is {}", i)
    }



}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn example() {

    let v = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(3.14),
    ];
    
}

fn test_string() {
    let mut s = String::new();

    let data = "initial contents"; // &str
    let s1 = data.to_string();  // String

    let s2 = String::from("initial contents");

    let mut foo = String::from("foo");
    let bar = String::from(" bar");
    // push_str 的参数是 &str，所以不会获取其所有权
    foo.push_str(&bar);
    println!("foo = {}, {}", foo, bar); // so 这里还能使用 bar

    
    // push
    let mut lo = String::from("lo");
    lo.push('l');
    println!("{}", lo);

    // + 
    // + 前为 String 类型，+ 后需要为字符串切片（也就是字符串引用类型）
    let mut s11 = String::from("hello");
    let mut s22 = String::from(" world");
    let s33 = s11 + &s22;
    println!("{}", s33);
    //println!("{}", s11); // + 拼接后 s11 不可以再使用了
    println!("{}", s22);


    let a = String::from("a");
    let b = String::from("b");
    let c = String::from("c");

    // let abc1 = a + "-" + &b + "-" + &c;
    // println!("{}", abc1);

    // format 不会取得任何参数的所有权
    let abc: String = format!("{}-{}-{}", a, b, c);
    println!("{}", abc);

    let hello = "Здравствуйте";

    // 一个字母（Unicode 标量值）占

    println!("Bytes");
    for byte in hello.bytes() { // 以字节形式
        println!("byte {}", byte);  // 208 ...
    }

    for char in hello.chars() { // 以 Unicode 标量值形式
        println!("char {}", char);  // З ...
    }

    let sStr = &hello[0..4]; 
    println!("{}", sStr);

    // let ss = &hello[1]
    // print!(ss)

}

fn test_hash_map() {

    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("tom", 40);
    println!("{:#?}", scores);

    let team = vec![String::from("Blue"), String::from("Yellow")];
    let init_score = vec![10, 50];
    let scores2: HashMap<_, _> = team.iter().zip(init_score.iter()).collect(); 
    println!("{:?}", scores2);


    let field_key = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // map.insert(field_key, field_value);
    map.insert(&field_key, &field_value);

    // 这里会报错，因为 field_key，field_value 已经 Move 发生了所有权改变
    println!("{}, {}", field_key, field_value);


    let mut score_map = HashMap::new();
    score_map.insert(String::from("Blue"), 10);
    score_map.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = score_map.get(&team_name);

    match score {
        Some(s) => println!("team score is {}", s),
        None => println!("team is not exist"),
    }

    for (k, v) in &score_map {
        println!("key={}, value={}", k, v);
    }

    update_hash();
}

fn update_hash() {

    let mut score_map = HashMap::new();
    score_map.insert(String::from("Blue"), 10);

    /*
        entry 方法判断 Yellow 这个 key 是否存在于 score_map
        如果不存在我们就插入 50
     */
    let e = score_map.entry(String::from("Yellow"));
    // e is  VacantEntry("Yellow"), VacantEntry 就是空, 即 Yellow 这个 key 不存在
    println!("e {:#?}", e);
    e.or_insert(50);
    score_map.entry(String::from("Blue")).or_insert(50);



    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 第一个 word 因为 key 不存在，会插入值为 0
        // 第二次 word key 已经存在，entry 直接返回 V 也就是 0
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);


}