fn main() {
    let mut s = String::new();

    let data = "contents";

    let s = data.to_string();
    let s = "contents".to_string();
    
    // This and the above syntax are equivalent
    let s = String::from("contents");


    let mut s = String::from("foo");

    let s2 = "bar";
    s.push_str(s2);

    println!("{}", s);

    s.push('c');

    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // + operator takes ownership of base string, but only has reference to concatenated strings

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe); // format! takes no ownership of any input strings
    println!("{}", tic_tac_toe);

    // let t = tic_tac_toe[0]; // wont compile

    // A String is a wrapper over a vec<u8>

    // for c in "string".chars() {
    //     println!("{}", c);
    // }

    for b in "bytes".bytes() {
        println!("{}", b);
    }



    // so basically, strings are complicated. If I want to work with
    // the word "Здравствуйте", I can't just index it to get characters.
    // Each character in that word is represented by 2 bytes, not one,
    // so if I index it, I'll only get half a character which will error.
}
