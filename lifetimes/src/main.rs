use std::fmt::Display;

// The 'a means the string slice that part references must live at least as
// long as the struct itself
struct ImportantExceprt<'a> {
    part: &'a str
}

impl<'a> ImportantExceprt<'a> {
    // First elision rule applies here. Lifetime implied
    fn level(&self) -> i32 {
        3
    }

    // Third elision rule applies here. Since there are multiple params
    // usually the compiler wouldn't know what lifetime to apply
    // to the output. However, since one of the params is a reference
    // to self, the compiler applies self's lifetime to the output.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// This is not allowed without the lifetime param
// impl ImportantExceprt {
//     fn num() -> i32 {
//         3
//     }
// }

fn main() {

    // This wont compile. r's reference to x's value does not live long enough
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // x's value is deallocated before r uses it, so any code that uses
    // r wont work properly. x has gone out of scope.

    // println!("{}", r);

    // The Rust compiler has a borrow checker, meaning it checks the
    // life time of variables and determines whether it will compile
    // the program based on that. In the above code, r has a longer
    // life than x. r tries to use x's value after x has gone out
    // of scope. Rust wont compile this.

    // This is the fix
    let r;

    let x = 5;

    r = &x;

    // println!("{}", r);


    let string1 = String::from("abcd");
    let string2;
    let result;

    {
        let string3 = "xyz";
        string2 = &string3;

        result = longest(string1.as_str(), string2);
    }

    // This line wont work if longest() returns a reference because
    // that reference will go out of scope in the inner curly brackets
    println!("Longest is: {}", result);

    // println!("{}", string2);

    // let result = longest(string1.as_str(), string2);
    // println!("Longest is: {}", result);

    // How to annotate lifetime params
    // &i32;
    // &'a i32;
    // &'a mut i32;

    let novel = String::from("Call me Ishmael! Four score and...");
    let first_sentance = novel.split(".").next().expect("No .");
    let i = ImportantExceprt {
        part: first_sentance
    };

    // Static lifetimes mean the reference can live for
    // the entire duration of the program.
    // The text of string literals is stored directly in the
    // programs binary, so its always available.
    // As far as I can see, there's no good reason to ever
    // declare a variable's reference as as static lifetime.
    // This seems like something that's good to be aware of
    // but really doesn't require my interaction as the
    // programmer
    let s: &'static str = "I have a static lifetime";
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

fn other_longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn arbitrary_lifetime<'completely_arbitrary>(x: &'completely_arbitrary str) -> &'static str {
    // Here's a good example of not using static lifetime,
    // or at least not explicitly calling things static
    // that arent static. This function wont compile. Why?
    // Because I'm trying to return a reference to num, which
    // goes out of scope as soon as this function ends. So I'd
    // be returning a dangling reference I believe is the
    // preferred nomenclature. So even though the refernce I'm
    // returning I've declared to have a static lifetime, it
    // doesnt. So that's why it seems silly to ever declare
    // that something has a static lifetime.
    
    // let num = 1;
    // let y: &'static i32 = &num;
    // y

    // This works, but string literals are already have a static
    // lifetime. So I don't understand why I would ever declare
    // a variable to have a static lifetime manually...
    let string = "literal";
    let y: &'static str = &string;
    y
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
Lets talk about lifetime elision...

Lifetime elision is the process the compiler goes through to figure out
implicit lifetimes, making my job as the programmer easier.

The compiler has 3 rules it follows to figure out implicit lifetimes.

1. Apply a lifetime to each parameter of a function
fn func(x: &str)
|
v
fn func<'a>(x: &'a str)

2. If there is one input parameter, apply that params lifetime to all output references
fn func(x: &str) -> &str
|
v
fn func<'a>(x: &'a str) -> &'a str

3. If there are multiple input lifetime params, but one of them is &self
or &mut self, apply that lifetime to all output params.

This cleans up methods nicely.



So looking at rules 1 and 2, basically a summary of those rules is:
if there is one input parameter, output params will have the same lifetime
and can therefore be implied. If there are multiple input params, the
compiler doesn't know which lifetimes to apply to the outputs, so it
cant imply the lifetimes and they must be explicitly stated.
*/