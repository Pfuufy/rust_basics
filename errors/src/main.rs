// use std::fs::File;
// use std::io::ErrorKind;
// use std::io;

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;


pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100. Got value {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    } 
}

fn main() -> Result<(), Box<dyn Error>> {
    guess_the_num();

    // panic!("Mwa hahahahahah!!! I am melon lord!!!");

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Mais non!!! {:?}", e)
    //         },
    //         call_it_what_you_want => {
    //             panic!("Some other issue, I guess: {:?}", call_it_what_you_want)
    //         }
    //     }
    // };

    // // Apparently, a more EXPERIENCED Rustacean might write something like this...
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Couldn't do it hoss: {:?}", error);
    //         })
    //     } else {
    //         panic!("Still can't do it homie");
    //     }
    // });

    // unwrap() automatically calls Result::Ok or Result::Err based
    // on the response of trying to open the file.
    // let f = File::open("hello.txt").unwrap();

    // expect() is like unwrap() except you can specify the error message
    // let f = File::open("hello.txt").expect("It dont exist maine");

    let data = read_from_file();

    let f = File::open("hello.txt")?;

    Ok(())
}

fn read_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }

    // What the ? operator does is run any errors through the from() function.String
    // The from() function converts errors of many different kinds into
    // the kind specified by the return type of the function. Rust will
    // do this automatically as long as the errors implement the from()
    // function.


    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Even further reduced
    // let mut s = String::new();

    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // Ok(s)

    // Yet even further...
    // Basically, all of the above operations are so common in Rust that
    // they created this method which does all that for you.
    fs::read_to_string("hello.txt")
}

fn guess_the_num() {
    // Pretend to receive a user's guess right here. Say we must
    // guarantee that their guess is between 1 and 100. We can
    // use the Guess struct created above to guarantee that, rather
    // than just using a generic i32 or whatever. This way, we
    // can ensure that any futher code that relies on the number
    // being between those numbers does not have to check to values
    // itself, saving writing and performance.

    // The compiler won't catch this, but it will fail during run time
    let user_input = Guess::new(101);
    
    // Operations on input...
}


/*
Now, when should you use panic! and Result?

panic! should be used when the application has encountered that is unrecoverable.
When the application encounters these errors, it should not be allowed to continue
in any capacity because subsequent code relies on the integrity of its predecessors.
Compromising that input quality can result in bad, had to track down bugs.

Result should be used when errors are normal or expected, for example when trying
to open a file that may or may not exist. If the application tries to open
a file that doesn't exist, it would be annoying if it just crapped itself every time.
Rather, it should create the file in stead.

panic! completely halts execution, while Result allows you to handle
it gracefully.
*/