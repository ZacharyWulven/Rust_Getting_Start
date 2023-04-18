use std::{cell::{RefCell, Ref}, rc::{Rc, Weak}, borrow::Borrow};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use crate::List:: { Cons, Nil };

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
// --------------------------

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    leak();

    node()
}

fn leak() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b create = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    // 死循环，stack overflow
    //println!("a next item = {:?}", a.tail());

}

fn node() {
    println!("node fn begin--------------------");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
        println!("2 leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("2 leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("2 branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    }

    println!("3 leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    println!("3 leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}