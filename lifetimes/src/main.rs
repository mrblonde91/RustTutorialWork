//10.3 Lifetimes
fn main() {
    //Lifetimes are compared `b is shorter than `a so invalid, removing braces would resolve issue.
    {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
    }         
}
