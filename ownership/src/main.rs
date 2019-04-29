fn main() {
    complex_test();
    invalid_reference();
    clone_heapreference();
    stackvaluesalwaysavailable();
    
    let x = String::from("Test owner");
    takes_ownership(x);

    println!("{}", gifts_ownership());
    
    let passmeback = String::from("Gimme back");

    let passedback = take_and_give_back(passmeback);
    println!("{}",passedback);

    let sample = String::from("Take this string and give me it's length");
    let result = calculate_length(sample);

    println!("Sample:{}, length:{}", result.0, result.1 );
}

fn complex_test(){
    // literals are immutable, size known at compile time
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    }

fn invalid_reference(){
    let s1= String::from("Hello");
    let s2 = s1;
    println!("S1 fails but s2 will succeed with {}. S1 got reallocated to s2", s2);
}

fn clone_heapreference(){
    let s1= String::from("Hello");
    let s2 = s1.clone();
    println!("Copy a complex type on heap, Do a clone. S1:{}. S2:{}", s1,s2);
}

fn stackvaluesalwaysavailable(){
    let x = 6;
    let c = x;

    println!("x:{}, c:{} on the stack",x,c);
}

fn takes_ownership(x: String){
    println!("  If value {}, is passed into a function. 
    The function takes ownership. This does not apply to primitives that copy applies to.", x);
}

fn gifts_ownership() -> String {
    let s = String::from("Returning will give ownership back.");
    s
}

fn take_and_give_back(x : String) -> String{
    x
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}