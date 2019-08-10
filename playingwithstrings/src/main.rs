fn main() {
    createstring("samplestring");
    iteratingthroughstring("evaluate this shit");
}

fn createstring(string_to_manipulate:&str){
    let mut samplestring = String::new();
    // string_to_manipulate.to_string() can be used to convert to String or below
    samplestring = String::from(string_to_manipulate);
    // To expand string,doesn't take ownership
    samplestring.push_str("bar");

    // To push a character
    samplestring.push('a');

    // Concatenation, will lose ownership of s1 and s2 needs to 
    // be a reference because of add's signature.
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{}, concatenated string: {}", samplestring, s3);
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let simplifiedConcat = format!("{}-{}-{}", s4, s5, s6);
    println!("Use of format! : {}", simplifiedConcat);
    //This will slice first two bytes, characters can be greater a byte.
    let slicedstring = &simplifiedConcat[0..2];
    println!("Slice string:{}", slicedstring);
}

fn iteratingthroughstring(string_to_process: &str){
    //can loop through characters of a string, cannot use string index due to size
    for c in string_to_process.chars(){
        println!("{}", c);
    }

    //Iterate through individual bytes
    //Readup on grapheme clusters https://craftofcoding.wordpress.com/tag/grapheme-cluster/
    for c in string_to_process.bytes(){
        println!("{}", c);
    }
}