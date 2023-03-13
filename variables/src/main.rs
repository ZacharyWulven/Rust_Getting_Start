const MAX_HEIGHT: u32 = 100_000;
fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("the x is {}", x);

    x = 6;
    println!("the x is {}", x);

    println!("MAX_HEIGHT  is {}", MAX_HEIGHT);

    let y = 5;
    let y = y + 1;

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess {}", guess);
    
    let f = 2.0;

    let emoj = '🥰';
}
