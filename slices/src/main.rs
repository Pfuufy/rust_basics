fn main() {
    let sentance = String::from("There are four words");

    let first = first_word(&sentance[..]);
    let second = second_word(&sentance[..]);

    // sentance.clear(); // wont compile

    println!("first: {}", first);
    println!("second: {}", second);

    // let hello_world = "hello world";

    // let _hello = &hello_world[0..5];
    // let hello = &hello_world[..5];
    // let _world = &hello_world[6..11];
    // let world = &hello_world[6..hello_world.len()];
    // let hello_world2 = &hello_world[..];

    // println!("{}", hello);
    // println!("{}", world);
    // println!("{}", hello_world2);

    let array = [1, 2, 3];

    let arr1 = &array[..2];

    println!("{}, {}", arr1[0], arr1[1]);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut found_first = false;
    let mut first_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if found_first == true {
                return &s[first_index + 1..i];
            }

            found_first = true;
            first_index = i;
        }
    }

    return &s[..];
}
