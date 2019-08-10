// 8.3 - Storing key in hashmap
use std::collections::HashMap;
fn main() {
    sample_dictionary();
    try_to_add();
}

fn sample_dictionary() {
    let mut scorelist = HashMap::new();

    scorelist.insert(String::from("red"), 10);
    scorelist.insert(String::from("yellow"), 76);
    let colour = String::from("orange");
    let result = 12;
    scorelist.insert(colour, result);
    let key = "orange";
    //THE below will return an option
    match scorelist.get(key){
        Some(v) => println!("Value of key: {} and value is {}", key, v),
        None => println!("No matching result")
    }
    for (key,value) in &scorelist {
          println!("{} - {}", key, value);
    }
    //The below would fail as lost ownership of the string, doesn't occur for types that implement copy
    //println!("{}", colour);
}

fn try_to_add(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
   
    //Now try to add and only add if doesn't exist, gets entry or insert
    scores.entry(String::from("Blue")).or_insert(5);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

        //Word counter and updating values
    let sentence = "The world and our shit is not enough and. We Saw it trough. The truth is";

    let mut map = HashMap::new();
    for word in sentence.split_whitespace(){
        //Insert zero if new value or increment existing otherwise.
        let count = map.entry(word).or_insert(0);
        //Update reference to value
        *count +=1;
    }

    println!("{:?}", map);
}

fn sample_zippeddictionary(){
    let teams = vec![String::from("Man utd"), String::from("Liverpool")];
    let initialscores = vec![3,4];

    //Zip values together then compile to hashmap, bit like linq
    let scores: HashMap<_, _> = teams.iter().zip(initialscores.iter()).collect();
}
