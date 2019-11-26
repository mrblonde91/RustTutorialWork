//15.01 boxes
enum List {
    // to count references
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    boxup();
    test_con();

    //15.04
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    //reference gets cloned and they get counted in Rc, not a deep clone
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope  {}", Rc::strong_count(&a));
}

//boxes will store values on the heap, and pointer stored on stack
fn boxup(){
    let b = Rc::new(5);
    println!("B:{}",b);
}

fn test_con(){
    let list = Cons(1, 
        Rc::new(Cons(2, 
            Rc::new(Cons(3,
                Rc::new(Nil))))));
}