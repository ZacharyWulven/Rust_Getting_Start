trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human's fly");
    }
}

//-------------------------------------------

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn test_dog() {
    println!("A baby dog is called a {}", Dog::baby_name());

    // 这样就会报错，因为无法知道使用哪个类型的 Animal 实现
    //println!("A baby dog is called a {}", Animal::baby_name());

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

}


fn main() {
    let person = Human;
    person.fly();

    // 调用 Human 的 Pilot 实现
    Pilot::fly(&person);
    
    Wizard::fly(&person);

    test_dog();
}
