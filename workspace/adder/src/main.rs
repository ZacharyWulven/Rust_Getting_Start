use add_one;
use rand;
use add_two;

fn main() {
    let n = 10;
    println!("Hello {} plus one is {}!", n, add_one::add_one(n));

    println!("Hello {} plus two is {}!", n, add_two::add_two(n));

}



