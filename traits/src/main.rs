use std::fmt::Display;

pub trait Summary {
    // fn summarize(&self) -> String;

    // Default method
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
    
    fn summarize_author(&self) -> String;
}

pub trait UrMom {

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// Use the default method rather than implementing our own
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}



struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is {}", self.x);
        } else {
            println!("The largest number is {}", self.y)
        }
    }
}



// std library does this. You can implement a trait for a 
// type that implements another trait conditionally. This
// is called blanket implementations.
// impl<T: Display> ToString for T {

// }












fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: "gabor".to_string(),
        content: "Elvis is on the beach".to_string(),
        reply: true,
        retweet: false
    };

    println!("New tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
}

// This can only be called by types that implement
// the Summary trait. This is called trait bound syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This is equivalent to the above, but the above is
// syntax sugar for this function
fn long_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Both params can be different types, as long as they
// implement Summary
fn other_notify(item1: &impl Summary, item2: &impl Summary) {

}

// If we wanted to force both params to be of the same
// type, we would have to write the function like this
fn fourth_notify<T: Summary>(item1: &T, item2: &T) {

}

fn two_traits_notify(item: &(impl Summary + UrMom)) {

}

fn also_two_traits_notify<T: Summary + UrMom>(item: &T) {

}

// Having too many trait bounds can make a function cluttered
// and hard to read. Rust has a better syntax for this.
fn some_function<T: UrMom + Clone, U: Clone + UrMom>(t: &T, u: &U) -> i32 {
    1
}

fn some_better_function<T, U>(t: &T, u: &U) -> i32
    where T: UrMom + Clone,
          U: Clone + UrMom
{
    1
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: "the earl slim shady".to_string(),
        content: "please stand up".to_string(),
        reply: true,
        retweet: true
    }
}