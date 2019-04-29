fn main() {
    let mut valuetoretain = String::from("Reference retained");

    let length = do_something(&valuetoretain);

    println!("String: {}, length:{}", valuetoretain, length );

    modify_a_reference(&mut valuetoretain);
    println!("modified it: {}", valuetoretain);
}

fn do_something(x: &String) -> usize{
    x.len()
}


fn modify_a_reference(x: &mut String){
    x.push_str(" was modified but only can be borrowed one at a time. This prevents data races.
    Curly brackets can create a new scope, remember this!");
}

// fn dangle()-> &String{
//     let s =  String::from("hello");
//     &s
// }
// This will fail as it will cease to exist after it returns so why return a reference?