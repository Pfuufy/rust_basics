use rand::Rng;

// use std::{cmp::Ordering, io};

use std::io::{self, Write};

// These are the same 

// use std::io;
// use std::io::Write;

// glob operator
use std::collections::*;


mod front_of_house;
pub use crate::front_of_house::hosting;



// pub fn eat_at_restaurant() {
    // // absolute path
    // let function = crate::front_of_house::hosting::add_to_waitlist;

    // function();

    // // relative path
    // front_of_house::hosting::add_to_waitlist();
// }

fn no_mod_function() {

}

mod back_of_house {
    mod chef {
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order(); // this is more concise than the function call right below
            crate::back_of_house::serve_order();
            super::super::no_mod_function();
            crate::no_mod_function();
        }
    
        fn cook_order() {
    
        }
    }

    fn serve_order() {

    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {

        // this summer function is required because the Breakfast struct has a private field
        // Without this function, the struct could never be created because there would
        // be no way to set that field.
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }

        pub fn repeat_order(&self) {
            println!("You order toast from {} bread, and your fruit is {}", self.toast, self.seasonal_fruit);
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

// This is like creating an OS symlink
// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

// we can also do this...
use crate::front_of_house::hosting::add_to_waitlist;
// and just make straight calls to add_to_waitlist,
// but it is more clear to bring parent module into scope
// and call it that way

// however, when using structs, enums, and other items,
// it is more idiomatic to use the full path
use std::collections::HashMap;
// let mut map = HashMap::new();
// There's no particular reason for this, this is just the pattern
// that's emerged from people using Rust

// The exception to this is when we bring in two things from
// different modules that have the same name
use std::fmt;
use std::io;
// fmt::Result;
// io::Result;

// Buuuuuuut we can also do this...
use std::fmt::Result;
use std::io::Result as IoResult;

// we can re-export stuff by importing it and then making it public
// importing stuff by default makes it private within the current scope
pub use crate::front_of_house::serving;




pub fn eat_at_restaurant() {
    // absolute path
    let function = crate::front_of_house::hosting::add_to_waitlist;

    function();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a summer breakfast meal with rye bread
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mide about type of bread
    meal.toast = String::from("Sourdough");
    println!("I'd like {} toast please", meal.toast);

    // We cant compile if we uncomment this line because seasonal_fruit
    // is private and we can't change it
    // meal.seasonal_fruit = String::from("canteloupe");

    // this also doesnt work
    // println!("I'll guess the {} is ok", meal.seasonal_fruit);

    meal.repeat_order();

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    // I can do this because of the use statement above
    hosting::add_to_waitlist();
}