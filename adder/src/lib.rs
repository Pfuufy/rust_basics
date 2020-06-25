#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1  {
            panic!(
                "Value must be less than 100. Got value {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Value must be greater than 1. Got value {}",
                value
            )
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greet(name: &str) -> String {
    format!("Hello {}!", "name")
}

#[cfg(test)]
mod caramel {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four!"))
        }
    }

    #[test]
    // Expected string is a substring of the expected panic message
    #[should_panic(expected = "Value must be less than 100")]
    fn greater_than_100() {
        Guess::new(300);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greet("Bernard");
        assert!(
            result.contains("Bernard"),
            "Greeting did not contain the name. Value was '{}'",
            result
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // fn get_rectangles() -> (Rectangle, Rectangle) {
    //     let larger = Rectangle {
    //         width: 10,
    //         height: 20
    //     };

    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 10
    //     };

    //     (smaller, larger)
    // }

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let (smaller, larger) = get_rectangles();

    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let (smaller, larger) = get_rectangles();

    //     assert!(!smaller.can_hold(&larger));
    // }

    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn how_about_them_apples() {
    //     assert_ne!(2 + 2, 4);
    // }

    // #[test]
    // fn utter_failure() {
    //     panic!("Mayday mayday!");
    // }
}
