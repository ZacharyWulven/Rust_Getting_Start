use add_one;

fn main() {
    println!("Hello, world!");
    let n = 10;
    println!("Hello {} plus one is {}!", n, add_one::add_one(n));
}
