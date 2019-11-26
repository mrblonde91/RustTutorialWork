#[derive(Debug)]
enum List{Cons(Rc<RefCell<i32>>, Rc<List>),
Nil,}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
//15.05 Modifying value after fact
fn main() {
    let value = Rc::new(RefCell::new(10));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(8)), Rc::clone(&a));

    *value.borrow_mut() +=10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
