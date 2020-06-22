struct User {
    username: String, // the pieces of data in a struct are called fields
    email: String,
    sign_in_count: u64,
    active: bool
}

// struct Ephemeral_User {
//     username: &str
// } // This wont compile. Lifetimes must be specified for the compiler to allow a reference in a struct.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// Even though the Color and Point structs both define tuples of the same type,
// a function that accepts a Color will not accept a Point. That would defeat
// the purpose of tuple structs.

fn main() {
    println!("Hello, world!");

    // let mut user1 = User {
    //     email: String::from("email"),
    //     username: String::from("username"),
    //     sign_in_count: 1,
    //     active: false
    // };

    // user1.active = true;

    let user1 = create_user(String::from("email"), String::from("username"));

    println!("{}", user1.active);

    // let user2 = User {
    //     email: String::from("user2"),
    //     username: String::from("username2"),
    //     sign_in_count: 2,
    //     active: true
    // };

    let user2 = User {
        email: String::from("user2"),
        username: String::from("username2"),
        ..user1
    }; // struct update syntax, like spread syntax in JS!


}

fn create_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        sign_in_count: 1,
        active: false
    };
}

fn take_color(color: Color) {
    println!("{}, {}, {}", color.0, color.1, color.2);
}

fn take_point(point: Point) {
    println!("{}, {}, {}", point.0, point.1, point.2);
}
