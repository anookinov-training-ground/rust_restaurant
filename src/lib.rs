// package -> crate(s) -> module(s) -> item(s) e.g. functions, structs, enums

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

mod front_of_house; // tells Rust to load the contents of the module from another file with the same name as the module

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting; // use with an absolute path
// use self::front_of_house::hosting; // use with a relative path
// use crate::front_of_house::hosting::add_to_waitlist; // ok but not recommended as it's not an idiomatic way (unclear as to where function is defined)

// pub use crate::front_of_house::hosting; // re-exporting names with pub use

pub fn eat_at_restaurant() {
    // Absolute path - opinionated preference
    // crate::front_of_house::hosting::add_to_waitlist();
    
    hosting::add_to_waitlist(); // with use keyword
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}