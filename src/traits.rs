// these are essentially interfaces

use std::fmt::{Display, Debug};

// defining a trait - those structs which implement this trait must contain and impl
// of fn summarize
pub trait Summary {
    fn summarize(&self) -> String;
}

//  for example
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}. by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

pub fn using_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("this is a tweet about horses"),
        reply: false,
        retweet: false,
    };

    // You can call summarise just as any other impl
    println!("1 new tweet: {}", tweet.summarize());
}

// restriction
// implement a trait on a type only if either trait or the type is local to our crate
// you cannot pull a trait from somewhere and implement it on a type also pulled from somewhere
// enforces a type of safety. We can implement foreign traits on local struct, and implement a
// local trait on a foreign struct. Orphan rule, you can't break other people code.


// default implementation

pub trait DefaultSummary {
    fn summarize_author(&self) -> String;
    // default implementations can reference yet to be imlemented methods
    fn summarize_with_default(&self) -> String {
        format!("(Read more...), {}", self.summarize_author())
    }
}

impl DefaultSummary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn use_default_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("this is a tweet about horses"),
        reply: false,
        retweet: false,
    };
    tweet.summarize(); // (Read more...), @horse_ebooks
}

// Traits as params

// using the impl trait syntax $impl we can pass in any struct that implements Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


// trait bound syntax
// $impl Trait is shorthand for a generic form called trait bound syntax

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// while more vebose it can help with more complicated signatures

pub fn notify_complex_sig<T: Summary>(item1: &T, item2: &T) {
    // better than (item1: &impl Summary, item2: &impl Summary){ ...
}

// multiple traits can be specified using the + syntax
pub fn notify_with_plus_sig(item: &(impl Summary + Display)) {}

pub fn generic_alternative_to_above<T: Summary + Display>(item: &T) {}

// an alternative exists to trait bound syntax
// original
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

// with where clause
fn some_function_with_where<T, U>(t: &T, u: &U)
    where T: Display + Clone, U: Clone + Debug {}

// traits can be returned from functions

fn returns_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("this is a tweet about horses"),
        reply: false,
        retweet: false,
    }
}
// this is particularly useful with closures and iterators. These create types
// that only the compiler knows. impl Trait syntax lets you concisely specify that a
// function returns some types that implements the Iterator trait, without needing
// to write out a very long type

// you cannot return multiple traits from a function - a switch statement which returns
// either one type of implementation over another will not work. There is a workaround,
// covered later in the book


// Using trait bounds to conditionally implement methods

// trait bound impl block that uses generic types params can be used to implement methods
// conditionally for types that implement specific traits

// e.g. Pair<T> always implements the new function, but Pair<T> only implements the
// cmp_display method if T implements the PartialOrd trait that enables comparison,
// and the Display trait that enables printing


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y{
            println!("The largest member is = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

// it's also possible to conditionally implement a trait for any type that implements
// a trait

// implementation of a trait on any type that satisfies the trait bounds are called
// blanket implementations

// the standard library uses this blanket implementation of ToString
// any type that implements display can now call the to_string method
// impl<T: Display> ToString for T{
// }


