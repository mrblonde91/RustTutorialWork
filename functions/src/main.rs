fn main() {
    another_function(35);
    let x = 3;
    let y : i32 = {
        let x = 3;
        x+3
    };

    println!("The value of y is: {}", y);
    println!("The value expected value is: {}", expected_value());
}

fn another_function(x: i32){
    println!("The value of x is:{}", x);
}

fn expected_value() -> i32{
    42
}