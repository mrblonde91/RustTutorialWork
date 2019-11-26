//15.02
use std::ops::Deref;
fn main() {
    check_dereferenced();
    check_dereferenced_box();

    //Defer coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(value: &str){
    println!("Hello {}", value);
}

struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    //associated type for deref to use.
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn check_dereferenced(){
    let x = 5; 
    let y = &x;

    assert_eq!(x, 5);
    //derefence to follow back to value to compare.
    assert_eq!(*y, 5);
}

fn check_dereferenced_box(){
    let x = 5; 
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    //can do same dereference with a box
    assert_eq!(*y, 5);
}
