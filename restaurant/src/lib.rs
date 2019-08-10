
    fn serve_order(){}
    
    mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,// One member public, other is internal
    }
    pub enum Appetizer{
        Soup,
        Salad
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("Peaches"),
            }
        }
    }

    fn cook_order(){}
    }

//Reference to external file
mod front_of_house;
//Absolute, attach pub to start to reexport
pub use crate::front_of_house::hosting;
//Relative path
//use self::front_of_house::hosting;
//Aliasing example: use std::io::Result as IoResult;

pub fn eat_at_restaurant(){
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    //relative
    //front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    //Order breakfast
    let mut meal = back_of_house::Breakfast::summer("Brown");
    meal.toast = String::from("Rye");
    println!("I'd like breakfast with {} toast", meal.toast );

    //Below line can't run as field is private
    //meal.seasonal_fruit = String::from("peach")
    
    let order1 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
