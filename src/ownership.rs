/*
Stack and the heap
Whether or not something is allocated on the stack or the heap has major implications in rust.
Stack and heap are parts of memory which are available to code at runtime.
The stack stores values in the order it gets them, and removes the values in th opposite order.
All data on the stack must have a known fixed size at compile time - otherwise it is stored
on the heap.

This is a less organised section of memory. When data is put on the heap, a certain amount of
space is requested. The memory allocator finds an empty spot that is big enough, and returns
a pointer - the address of that location. This is called allocating, the process in which
memory is 'pointed' to by some address which is stored on the stack.
Accessing the heap is slower because you must follow a pointer to the memory location.
Jumping around in memory is not conducive to caching either.

When your code calls a function, the values passed to it (stack allocated pointer or value)
get pushed to the stack, and when the function is over, those values get popped off.

Ownership addresses several problems related to stack and heap allocation. Keeping track of what
parts of the code are on the heap, and cleaning up unused data on the heap.

Ownership rules
    1) Each value in rust has a variable that's is its owner
    2) There can only be one owner at a time
    3) When the owner goes out of scope, the value will be dropped
*/
pub fn this_main() {
    {                               // s not valid here, not year declared
        let s = "hello";      // value from here
        // stuff can be done with s here
    }                               // s is not longer valid

    // this is a heap allocated String type, unlike the string literals used in prev examples
    // double colon is an operator that allows us to namespace this particular from function
    // under the string type
    let mut s = String::from("hello"); // can be mutated
    s.push_str(", world!");
    println!("{}", s);

    // the immutable &str literal type can be stack allocated because it's size is known
    // and cannot change
    // String::from essentially requests a part of memory which can be used for our string
    // without a GC we would normally need identify when memory is no longer needed and
    // explicitly return it.

    // In rust memory is automatically returned once the variable that owns it goes out of
    // scope!
    {
        let s = String::from("hello");      // valid s
    }                                                 // drop is called and s is invalid

    // MOVING

    let x = 5;
    let y = x; // stack allocated x can be copied to y

    let s1 = String::from("hello");
    let s2 = s1;  // copy a pointer - both s1 and s2 point to same mem on stack

    // With naive scoping rules, if either s1 or s2 went out of scope, they would try
    // to clear up the heap, and leave a dangling pointer to no memory

    // borrowing rules prevent this, as ownership has now been passed to s2. This dictates
    // scoping rules

    // In this case any attempt to use s1 fails because s1 has been invalidated by the Move
    // println!("{}", s1); will not compile!

    // instead of a shallow copy, we speak of moves in rust because copying a pointer
    // invalidates the original owner of a value

    // CLONE
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // this will produce a deep copy

    // COPY
    let x = 5;
    let y = x;
    println!("{} {} ", x, y);

    // we don't have to call clone in this case because certain stack values implement
    // the Copy trait
    // a type cannot implement both Copy and Drop at the same time
    // (They can't have value and reference type behaviour at the same time)
    // ownership_and_functions();
    // return_values_and_scope();
    // references_and_borrowing();
    // mutable_references();
    mutable_immutable_ref();
    // let reference_to_nothing = dangling_ref();
    // this function's return type contains a borrowed value, but there is no value
    // for it to be borrowed from.
}

fn ownership_and_functions() {
    // OWNERSHIP AND FUNCTIONS
    let s = String::from("hello");
    let x = 5;

    takes_ownership(s);
    // s is not longer valid here

    make_copy(x);
    // x is still valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // drop is called on some_string

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}

// some_int goes out of scope

fn return_values_and_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello"); // s2 no longer valid
    let s3 = takes_and_gives_back(s2); // s2 is moves, but given back to s3
} // s3 and s1 dropped

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // comes into scope
    some_string // moves to calling scope
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // moves back to calling scope
}

// returning values can be a bit tedious if you just want to use a value and retain
// ownership
fn references_and_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // reference s1 no ownership taken
    println!("The length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize { // must take in a reference to a string
    s.len() // read only access, no ownership taken
    // references are immutable by default
}
// calculate_length is borrowing the reference of s

// the opposite of referencing is referencing *  see chapter 8 & 15

fn mutable_references(){
    let mut s = String::from("hello");
    // change(&mut s); // pass a mutable reference
    // one big restriction with mutable references!
    // you can only hve one mutable reference to a bit of data within a particular scope
    let r1 = &mut s;
    // let r2 = &mut s;
    println!("{}", r1); // this will fail due to multiple mutable refs if r2 in scope
}
fn change(some_string: &mut String){ // expected mutable reference
    some_string.push_str(", world"); //
}

// this prevents data races from occurring because only one bit of code can manipulate a
// mutable pointer - two

// It is also not possible to mix mutable and immutable references in the same scope

fn mutable_immutable_ref(){
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2); // this is fine
    let r3 = &mut s; // can cause issues
    println!("{}", r3); // this is fine
    // this will cause a compile error
    // println!("{} {} {}",r1, r2 ,r3 )
    // accessing any mutable references after the  mutable ones will cause an error

    // a references scope started from where it is introduced, and continues until
    // the last time that reference is used. Hence we can mix immutable and mutable references
    // in some circumstances like the above

}

// some languages allow you to create dangling pointers - a pointer that references a
// location in memory that may have been given to someone else, by freeing that memory but
// preserving the pointer to that memory
// fn dangling_ref() -> &String{
//     let s = String::from("hello"); // s owns value
//     &s // return pointer to nothing
// } // scoped drop

fn no_dangling_ref() -> String{
    let s = String::from("hello"); // s owns value
    s // return actual value
} // nothing is dropped here because ownership is moved to calling code
