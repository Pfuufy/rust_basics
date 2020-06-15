fn third_function() {
    println!("third_function()");
}

fn main() {
    println!("Hello, world!");
    // another_function(5, 10);
    third_function();

    let second_function = another_function;

    second_function(5, 10);

    let x = 5;

    let y = {
        let x = 1;
        x + 5
    };

    println!("x: {}", x);

    println!("{}", x + y);

    println!("fourth(): {}", fourth());

}

fn another_function(x: i32, y: i32) {
    println!("another_function()");
    println!("The value of x, y is: {}, {}", x, y);
}

fn fourth() -> bool {
    true
}