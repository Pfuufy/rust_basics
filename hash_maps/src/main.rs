use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Shirts"), 100);
    scores.insert(String::from("Skins"), 50);

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")
    ];
    let scores = vec![10, 9000];

    // This combines the two vectors into one vector of tuples of the data elements
    // with zip(), then converts that vector into a hash map with collect().
    // We can specify the key and value types as _ because Rust is able to infer
    // the types from the original vectors. However, it needs the HashMap explicit
    // typing because collect() can collect into many different data types.
    let team_scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    let key = String::from("Favorite color");
    let value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(key, value);

    // We can no longer use these variables because they do not implement the copy trait.
    // Moving them into the hash map has given it ownership over them.
    // Copyable values like i32 are copied into the hash map, and therefore
    // are still valid after being copied into the hash map.
    // println!("Key: {}", key);
    // println!("Value: {}", value);

    let yellow_score = team_scores.get("Yello"); // Intentionally mispelled to trigger None arm

    match yellow_score {
        Some(score) => println!("Yellow score {}", score),
        None => println!("Yellow has no recorded score")
    }

    for (team, score) in &team_scores {
        println!("{}: {}", team, score);
    }



    // Updating hash maps


    let mut scores: HashMap<String, i32> = HashMap::new();

    // Override existing property
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 20);

    // Insert new value if key has no existing value
    scores.entry(String::from("blue")).or_insert(30); // value not updated because there's an existing score for blue
    scores.entry(String::from("red")).or_insert(50);

    println!("{:?}", scores);


    // Update based on old value
    let text = "hello my baby, hello my darling";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
