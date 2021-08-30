use std::collections::HashMap;

pub fn this_main() {
    // stores mapping between key -> values

    // creating an empty hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    // data is stored on the heap
    // all keys must have the same, and values must have the same data type

    // can be constructed by iterating with a collect method on a vector of tuples
    // each tuple will constitute a key value pair
    // the collect method gathers data into a number of collection types, such as HashMaps
    // we can also use the zip method to create a list of tuples from two vectors

    let teams = vec![String::from("Blue"), String::from("Green")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams   // type annotation needed because
        .into_iter()         // you can collect into a number of
        .zip(initial_scores.into_iter())  // types
        .collect();

    // types that implement the Copy trait like i32 will be copied into the hashmap
    // for owned values like string they will be moved and the hashmap will be the owner of
    // the values

    let name = String::from("harry");
    let second_name = String::from("prior");

    let mut map = HashMap::new();
    map.insert(name, second_name);
    // name and second name are no longer valid, they have been moved


    // if references are used then the values that the reference point to must be
    // valid for as long as the hashmap is valid

    // Accessing values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);  // Some(&10)

    // iterating over hashmap
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    // updating hashmaps

    //Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    // Insert only if no value present
    // entry api takes the key, returning an enum called Entry
    // this represents a value that may or may not exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);

    // updating value based on old one

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

}
