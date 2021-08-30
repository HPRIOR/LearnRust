/*
Rust only has one type of string in the core language. The string slice str, usually
referenced in its borrowed form &str

String slices are references to some utf-8 encoded string data stored in memory

The String type provided by the standard lib is a growable, mutable, owned utf-8 encoded
string type

When strings are referred to when speaking about rust, it usually means both String and &str

*/

pub fn this_main() {
    // string initialisation
    let mut s = String::new(); // we can push strings onto here as it's mutable

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents"); // create string from literal


    // appending
    let mut s = String::from("foo");
    s.push_str("bar");        // appending a string slice - we don't take ownership
    // of the param

    let mut s1 = String::from("foo");
    let s2 = "bar";     // if this were an immutable borrow
    s1.push_str(s2);   // and this borrowed s2
    println!("s2 is {}", s2); // this would not compile (so we use string literals)

    let mut s = String::from("lo");
    s.push('l'); // push a single character


    // combining existing strings with the + operator
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved and therefore is now invalid
    // Signature of add method is:  fn add(self, s: &str) -> String;
    // we add a reference to the of the second string to the first string
    // we can only add &str to a String, not two String values
    // the compiler can coerce the &String into a &str turning &s2 -> &s2[..]
    // because add does not take ownership s2 will still be valid
    // but it does take ownership of self (it's not &self), therefore s1 will be moved
    // and is invalid

    // Using format! macro to combine more complicated strings
    let day = String::from("4");
    let month = String::from("June");
    let year = String::from("1992");

    let s = format!("{}-{}-{}", day, month, year);

    // Indexing

    // one does not simply index into a string in rust
    let s1 = String::from("hello");
    // let h = s1[0]; // will not compile

    // this is because of how rust stores strings in memory
    // string is a wrapper over Vec<u8>

    let hello = String::from("Hola");           // len == 4
    let hello = String::from("Здравствуйте");   // len == 24!
    // indexing into a string will not always correlate with a valid unicode scalar value

    // you must be specific and create a string slice containing particular bytes
    let s = &hello[0..4];

    // you can iterate over chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // or bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b); // unicaode scalar values bay be made up of more that one byte
    }
}