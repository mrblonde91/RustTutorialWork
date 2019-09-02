//11.01 Automated test 
// Performance testing only available in nightly builds atm.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

// #[cfg(test)] means test code won't make into binaries

#[cfg(test)]
mod tests {
    //Brings rectangle into scope
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width: 8, height:10};
        let smaller = Rectangle{width: 5, height:6};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{width: 8, height:10};
        let smaller = Rectangle{width: 5, height:6};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }

    fn it_works() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    //Can add custom message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain the expected name: `{}`", result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 300.")]
    fn greater_than_100(){
        Guess::new(300);
    }
}
