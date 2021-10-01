// implementing the dereference trait allows your to control the behaviour of the *
// operator. Often this is emulate the behaviour of a regular reference.


use std::ops::Deref;

fn dereference_operator(){
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y); <- this won't compile because can't compare between ref and val int
    // you need to 'follow the pointer' to the value before you can compare, using the
    // * dereference operator
}

fn dereference_with_box(){
    let x = 5;
    let box_y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *box_y);
}

// defining our own smart pointer

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref_my_box(){
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // need to impl Deref trait
}
