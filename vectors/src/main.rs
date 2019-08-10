fn main() {
    //Immutable
    let b =vec![2,52,52];
    //Mutable
    let mut c = Vec::new();
    c.push(2);
    c.push(5);

    //index access
    let third: &i32 = &c[1];

    //Illegal to do when referencing(immutable reference) same vector
    //c.push(5);
    println!("The third element of b is {}", third);

    //This will throw an out of bounds exception
    //let doesntexist : &i32 = &b[7];

    //This will return option of none
    let doesnotexist = b.get(7);
    // Match, this means we've gotta handle options
    match b.get(2){
        Some(third) => println!("The third element exists and is {}",third),
        None => println!("There is no third element!")
    }

    //iteration
    for i in &b{
        println!("Current item is {}", i);
    }

    //modifying mutable
      for i in &mut c{
          // * derefences value so it can be modified
          *i += 110;
          println!("Current item is {}", i);
    }



    // Enums to put different types in an vector
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Float(5.0),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Test string"))
    ];
}