///9.1 - Check toml where panic behaviour set
fn main() {
    //Panic is for unrecoverable errors
    //panic!("Panic in the disco")
    //Anothe example
    let v = vec![2,4,5];
    v[99];
    ///Backtrace can be used to track the origins of the error
    /// to turn on backtrace `SET RUST_BACKTRACE=1`
}
