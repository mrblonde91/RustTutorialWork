//! # My crate is a very very good crate
//! Sample documentation, this one generates summary documentation for a crate

///Adds one to value
///# Examples
/// ```
///let mut value = 1;
///let result = documentation_examples::add_one(1);
/// assert_eq!(2, result);
///```

pub fn add_one(value: i32) -> i32{
    value + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
