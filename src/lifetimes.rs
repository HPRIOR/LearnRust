// Every reference has a lifetime, usually implicit, which is the scope for which a
// reference is valid
// We must annotate lifetimes when the lifetimes of references could be related in different
// ways

// the main aim of lifetimes is to prevent dangling references, which cause a program to
// reference data other than the data it's intended to reference.

use std::fmt::Display;

pub fn dangling_ref() {
    let r; // not null, if no value assigned a compile error will occur
    {
        let x = 5;
        r = &x;
    }
    // println!("r: {}", r); will not compile
    // x has gone our of scope
    // Because r's scope is larger than x, we say it lives longer than x
    // To determine the validity of references in this way, rust uses a 'borrow checker'
}

pub fn borrow_checker() {
    let r;                // ---------+-- 'a
    {                           //          |
        let x = 5;         // -+-- 'b  |
        r = &x;                 //  |       |
    }                           // -+       |
    // println!("r: {}", r);    //          |
}                               // ---------+
// lifetimes annotated with 'a 'b

pub fn valid_lifetime(){
    let x = 5;             // ----------+-- 'b
    let r = &x;           // --+-- 'a  |
    println!("r: {}", r);       //   |       |
}                               // ----------+

// x has the lifetime of b so r is a valid references

// Generic lifetimes in functions

fn use_longest(){
    // two strings are references, because we don't want the function to take ownership
    // of its params
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // 'a will be the smaller of the lifetimes passed to the function
    // this ensures that whatever reference is returned from the function is
    // guaranteed to be valid!
    let result = longest(string1.as_str(), string2);
    println!("The long string is {}", result);
}

// we don't know whether the method will return a reference to x or y,
// we cannot look at the scopes as we have above to determine whether the reference we
// return will be valid
// We need to add generic lifetime parameters that define the relationship between the references
// so the borrow checker can perform its analysis
// Lifetime annotations don't change how long a reference lives. They are similar to generics
// in that params can accept any lifetime passed to it. The annotation specifies the
// relationship between the arguments to a function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // all references must have same lifetime
    if x.len() > y.len(){
        x
    } else {
        y
    }
}
// in practical terms this means that the value returned by the function will have at
// least the same lifetime as the the smaller lifetime of the two arguments


// syntax
// &i32         a reference
// &'a i32      a reference with an explicit lifetime
// &'a mut i32  a mutable reference with an explicit lifetime

// lone references don't tell you much
// if you pass in two arguments with the same lifetime, you are saying that both references
// must lives as long as that generic lifetime


fn use_longest_with_scope(){
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // this will not compile, because the reference returned by longest is limited
    // to the scope of the shortest lifetime - string2
    // println!("the longest string is {}", result);
}

// In some cases you do not need to specify the lifetimes of all parameters
fn longest_one_lifetime<'a>(x: &'a str, y: &str) -> &'a str{
    x // this is the only value that is returned so the validity of t is irrelevant
    // outside of this functions scope
}

// when returning a reference at least one of the parameters lifetimes must match the
// return value of the function

// Lifetime annotations in struct definitions

// previously we've only had a structs fields be owned by the struct, but it is possible
// structs to hold references. In this case we need to add a lifetime annotatino to the
// struct and the reference.
struct ImportantExcerpt<'a>{
    part: &'a str, // reference to a string slice
}

// an instance of ImportantExcerpt cannot outlive the reference to the field part

fn use_lifetime_struct(){
    let novel = String::from("some string about a novel");
    let first_sentence = novel.split('.').next().expect("could not find a '.");
    let i = ImportantExcerpt{
        part: first_sentence
    };
}

// Lifetime Elision

// certain deterministic patterns can be be annotated automatically by the compiler, and
// are implicit rather than explicitly annotated

/*
3 elision rules:
    1 - each parameter that is a reference gets its own lifetime parameter
    2 - if there is exactly one input lifetime parameter, that lifetime is assigned
        to all output lifetime parameters
    3 - if there are multiple input lifetime parameters, but one of them is &self or
        &mut self, the life time of self is assigned to all output lifetimes
 */

// this third rule means we don't have to assign lifetimes in method signatures very often
// lifetime are often just assigned to the lifetime of the struct

// where we declare and use lifetime params here depends on whether they're related to the
// struct fields, or the method params and return values

// inside an impl block, references might be tied to the lifetime of references in the
// structs fields, or they might be independent - however lifetime elision rules often
// make it so tht lifetime annotations aren't necessary in method sigs

impl <'a> ImportantExcerpt<'a> { // lifetimes here are needed due to the structs use of them (?)
    fn level(&self) -> i32{ // not required on self because of (third rule)
        3
    }
    // or here
    fn announce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part
    }
    // return type implicitly has the lifetime of self
    // fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &'a str
}

// Static lifetimes
// this is a special lifetime that can live for the entire duration of the program
// all string literals have the 'static lifetime

fn static_lifetime(){
    let s: &'static str = "I will live forever!";
}

// Generic type parameters, trait bounds, and lifetimes

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display {
    print!("Announcement! {}", ann);
    if x.len() > y.len(){
        x
    } else {
        y
    }
}




