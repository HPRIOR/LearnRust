pub fn this_main() {
    // slices let you reference a continuous sequence of elements in a collection and do not
    // have ownership

    // write a function that take a string and returns the first word it finds
    // we don't currently know a way to refer to part of a string, so lets return the
    // index of the end of the word
    let mut s = String::from("hello world");
    let index = first_word(&s); // immutable borrow ends here
    s.clear(); // s == "" // mutable borrow

    println!("{}", index); // index is now invalid for first_word
    // if we try to use index on s now to retrieve words, we will get an index error
    // this is where string slices try to solve, they are references to part of a string

    let s = String::from("hello world");

    // &str is string slice type, the most primitive type of string
    let hello = &s[0..5]; // 0-4
    let world = &s[6..11]; // 6-10
    let he = &s[..2]; // 0-2
    let lo_world = &s[3..]; // 3-end

    // the rules surrounding borrowing ensure that we don't get the same runtime errors
    // associated with the initial first word function

    let mut s = String::from("hello world");

    let word = better_first_word(&s[..]); // immutable borrow of s
    s.clear(); // mutable borrow
    // println!("{}", word); // immutable borrow used after mutable borrow error!

    // confusion: why doesn't the the first function create an immutable borrow effecting
    // the println!? Answer(?): With the first function the immutable borrow ends when
    // the function ends, but persists because we retain a reference to the string through
    // word

    // string literals are stored inside the binary. the type here is &str: it's a slice pointing
    // to that specific point of the binary. This is why string literals are immutable; they
    // are immutable references (&str)
    let s = "Hello, world!";

    // Slices as parameters
    // we can improve the signature of the better_first_word function by using &str instead
    // of &String - this allows us to use the function on both types of value
    // With a String we can pass a slice of the entire value

    let word = better_first_word(&s[..]); // passes in string literal
    let word = better_first_word("hello");

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..3]; // stores reference to &[1,2,3]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();//.as_bytes(); // convert to an array of bytes

    // iter returns an iterator over slice
    // enumerate returns an Option tuple containing Some index and value
    for (i, &item) in bytes.iter().enumerate() { // tuple destructuring
        // we recieve a reference back from iter().enumerate() so we use &
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return word
        }
    }
    &s[..] // return full string
}

