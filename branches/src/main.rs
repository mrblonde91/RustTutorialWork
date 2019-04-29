fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("Number is: {}", number);
    loop_example();
    for_example();
    forrange_example();

}

fn forrangereverse_example(){
    for number in (1..4).rev(){
        println!("Number is {}", {number});
    }
    println!("Lift off");
}


fn for_example(){
    let a = [2,52,52,2626,6,27,34,7,478,4884];
    
    for element in a.iter(){
        println!("Value of element is {}", element);
    }
}

fn loop_example(){
    let mut counter = 0;
    let result = loop{
                counter+=1;

        println!("Domo arigato {}", counter);
        if counter == 10{
            break counter * 2;
        }
    };
    println!("final result:{}",{result} );
}