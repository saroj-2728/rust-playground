use std::collections::HashMap;

fn main() {
    // Creating
    let mut scores = HashMap::new();

    scores.insert(String::from("TeamA"), 100);
    scores.insert(String::from("TeamB"), 434);

    // Accessing values
    let score_of_b = scores.get("TeamB").copied().unwrap_or(0);
    // .get() returns an Option(&Value), above statement uses .copied() to get an Option<i32> instead of Option<&i32> and then .unwrap_or(0) sets the value to 0 if it doesn't exist
    println!("Score of TeamB: {score_of_b}\n");

    // Iterating over values, same as vectors
    println!("Teams and their scores:");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let key = String::from("TeamC");
    let value = 12;
    scores.insert(key, value); // Takes ownership
    // println!("Key: {key}"); // Err: borrow of moved value
    println!();

    // Updating Hashmaps
    let mut hsmap = HashMap::new();
    hsmap.insert(String::from("S1"), 23);
    hsmap.insert(String::from("S1"), 235); // Value replaces the above one (overwrites)
    println!("{hsmap:?}"); // {"S1": 235}

    // Adding a Key and Value Only If a Key Isn’t Present
    hsmap.entry(String::from("S2")).or_insert(99); // .entry checks whether a key S2 has a value or not, if yes ignore, if no insert 99
    hsmap.entry(String::from("S1")).or_insert(23); // doesn't insert cause Key "S1" with value already exists, so prints {"S2": 99, "S1": 235}
    println!("{hsmap:?}\n");

    // Updating a Value Based on the Old Value
    // Eg. counting no. of each word in a sentence
    let sentence = "This is a sentence that is a long one.";
    let mut map = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Count of each word: ");
    println!("{map:?}");
}
