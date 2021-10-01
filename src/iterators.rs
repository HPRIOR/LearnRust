fn processing_some_numbers() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // responsible for iterating logic (when end, next)
    // iterators are lazy meaning they have no effect until you call methods that consume the
    // iterator

    // for loops consume iterators
    for val in v1_iter {
        println!("Got: {}", val);
    }
    // not necessary to make mutable as the loops takes ownership and makes it mutable
    // for us
}


// std lib def of iterator

pub trait StdIterator {
    type Item;
    // associated type
    fn next(&mut self) -> Option<Self::Item>;  // covered in chapter 19

    // this code says implementing the iterator trait requires that you also define an
    // Item type, and this Item type is used in the return type of the next method
    // e.g. The Item type will be returned from the iterator
}

pub fn manually_call_iter() {
    let v1 = vec![1, 2, 3];
    let mut v1_mut = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // needs to be mutable
    // next method changes internal iterator state to track location of iterator

    assert_eq!(v1_iter.next(), Some(&1)); // values returned are immutable references
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // if we want to create an iterator which returns owned values
    let mut v1_owned_iter = v1.into_iter();
    assert_eq!(v1_owned_iter.next(), Some(1)); // values returned are immutable references

    let mut v1_mut_iter = v1_mut.iter_mut();
    assert_eq!(v1_mut_iter.next(), Some(&mut 1));
}


// the iterator trait has a number of default implementation - 'consuming adapters' which
// use up the iterator by repeatedly calling next

fn iterator_consuming_adapters() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // takes ownership
    assert_eq!(total, 6);
    // <- can't use v1_iter after consumed by sum
}

// iterator adapters allow you to change iterators into different kinds of iterators

fn iterator_adapter() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect method required as
    // iterators are lazy and the closure will not be executed unless consumed
}

// using closures that capture their environment

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // captures shoe size from argument, takes ownership of Vector, returns new one
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// creating your own iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Associated type for the iterator
    // means iterator will return u32 value

    fn next(&mut self) -> Option<Self::Item> { // references iterator associated type
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// let mut counter = Counter :: new();
// counter.next == Some(1);
// counter.next == Some(2);
// counter.next == Some(3);
// counter.next == Some(4);
// counter.next == Some(5);
// counter.next == None;

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    // sum == 18
}











