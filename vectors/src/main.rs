fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);

    let third = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third_el) => println!("The third element is {}", third_el),
    //     None => println!("There is no third element")
    // }

    // let does_not_exist = &v[100]; // this panics if index is out of bounds. Use if indices out of bounds is unexpected and not ok
    // let also_does_not_exist = v.get(100); // this does not panic, but returns None. Use if indices out of bounds is to be expected and is ok



    // for i in &v {
    //     println!("{}", i);
    // }

    // for i in &mut v {
    //     *i += 50; // * is dereference operator
    //     println!("{}", i);
    // }


    // This is how we achieve having different types in a vector
    let row = vec![
        SpreadsheetCell::Int(500),
        SpreadsheetCell::Text(String::from("Yeehaw!")),
        SpreadsheetCell::Float(1.0),
    ];

    for el in &row {
        match el {
            SpreadsheetCell::Int(val) => println!("i32: {}", val),
            SpreadsheetCell::Float(val) => println!("f64: {}", val),
            SpreadsheetCell::Text(val) => println!("string: {}", val)
        }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
