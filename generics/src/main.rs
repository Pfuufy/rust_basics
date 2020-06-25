// struct Point<T> {
//     x: T,
//     y: T
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// // This stuff is only implemented on Points of type f32
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

struct Two_Types<T, U> {
    x: T,
    y: U
}

enum Types<T, E> {
    Ok(T),
    Err(E)
}

fn main() {
    let nums = vec![1, 782, 22];

    let largest_num = largest(&nums);

    println!("largest num: {}", largest_num);

    let chars = vec!['d', 'u', 'n'];

    let largest_char = largest(&chars);

    println!("largest char: {}", largest_char);

    let integer = Point {x: 30, y: 8934};
    let float = Point {x: 1.89, y: 7.34};

    // let one = Point {x: 100, y: 1.0}; // Types must match
    let two = Two_Types {x: 100, y: 1.0};



    let p1 = Point {x: 5, y: 4.5};
    let p2 = Point {x: "Hello", y: 'a'};

    let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];

    for el in list {
        if el > biggest {
            biggest = el
        }
    }

    &biggest
}


/*
But how does using generics affect runtime performance?

Rust does a process called monomorphization whereby when a
program is compiled, it takes generic types and generates
concrete types for them.

For example, if we examine the standard library's Option<T>
enum (Some and None), we can see how concrete types are
created for code that uses it.

let integer = Some(5);
let float = Some(5.0);

Rust takes these and figures out that their types are i32 and
f64, respectively. Then it makes concrete enums for these types.

enum Option_i32 {
    Some(i32),
    None
}

enum Option_f64 {
    Some(f64),
    None
}

let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);

Doing this preserves runtime performance while allowing
us to not have to define several types manually. Yay!
*/
