// vectors can store multiple values next to each other in memory
pub fn this_main() {
    // creating a new empty vector
    let v: Vec<i32> = Vec::new();
    // in this case we need type annotation because the type of value cannot be inferred
    // we can use the vec! macro as a shorthand syntax to init an array
    let v = vec![1, 2, 3]; // will infer types based on values

    // type will also be inferred here
    let mut v = Vec::new(); // must be declared mutable to add/remove items
    v.push(1);
    v.push(2);
    v.push(3);

    // vectors are freed when they go out of scope, along with their elements
    // just as any other struct
    {
        let v = vec![1, 2, 3, 4];
    } // v is freed
    // things get more complicated when references are involved...

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // this will cause a panic if index not found

    println!("The third element is {}", third);

    match v.get(2) { // v.get return Option<&i32>
        None => {}
        Some(third) => println!("The third element is {}", third)
    };

    // borrowing rules apply! Borrow checker ensures references to vectors remain
    // valid

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow first element
    v.push(6); // this is a mutable borrow
    // println!("the first element is: {}", first);     // this makes the immutable
    // valid after mutable borrow

    // Iterating over a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Iterating and modifying
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // dereference i so that we 'follow' the pointer to the value
    }

    // Enums and variable typed vectors
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}