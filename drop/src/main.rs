use std::mem::drop;
//15.03
//Drop is code to run when value goes out of scope
fn main() {
    let c = CustomSmartPointer{data: String::from("My data")};
    //have to use a memory drop to get rid of value early
    drop(c);
    let d = CustomSmartPointer{data: String::from("is here")};
    println!("CustomSmart pointers created");
}

struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    //drop can't be called explicitly
    fn drop(&mut self){
        println!("Dropping smart point with data `{}`", self.data);
    }
}