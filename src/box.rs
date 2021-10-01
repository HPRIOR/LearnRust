
// Box<T>
// create a reference instead of a value, no overhead apart from heap allocation

//Why?
// when a type size cannot be known at compile time and you want to use a value of that
// type in a context that requires an exact size

// when a large amount of data needs to transfer ownership but not be copied

// when you want to own a value and you care only that it's type implements a trait
// rather than being a specific trait


fn storing_data_on_heap(){
    // this would never really happen as i32 values should always be stored on stack
    let b = Box::new(5);
    println!("b = {}", b);
}

// Recursive types

// this types size cannot be known at compile time, boxes have a known size so we can
// use them to store a recursive type
// cons list, a common datatype in functional programming languages
// each item in a cons lists contains two elements, the value of the current item, and the
// next item.

enum List{
    Cons(i32, Box<List>),
    Nil
}

fn use_cons(){
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
}

// to compute the size of a non recursive type:
// e.g. the message enum

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write (String),
    ChangeColour(i32, i32, i32) // largest variant will determine the size of space needed
}

// If we were to do this in the recursive case we would end up in an infinite loop, trying
// to find out the size of List
// we can add a level of indirection to this process by Boxing the value of the List
// because Box is a pointer, it always has the same size

// Box<T> implements the Deref trait which allows Box<T> values to be treated like references,
// When a Box<T> goes out of scope its implementation of Drop will also clean up the heap
