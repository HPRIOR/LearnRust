/*
Anonymous functions that can capture their environment
You can save these in a variable, or pass as arguments to other functions.
You can create a closure in one place and then call the closure to evaluate it in a
different context. They can capture values from the scope in which they're defined.
 */

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn simulated_expensive_calc(intesity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intesity
}

fn generate_workout(intesity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intesity
    });
    if intesity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intesity));
        println!("Today, do {} situps", expensive_result.value(intesity));
    } else {
        if random_number == 3 {
            println!("Take a break today: Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes", expensive_result.value(intesity))
        }
    }
}

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specific_value, simulated_random_number);
}


fn one_type_inference_per_closure_def() {
    let x = |x| x;
    x(5);
    // x("5");  <- won't compile
}

fn pass_closure() {
    let f = |x| x;
    accept_closure_int(f);
    // accept_closure_string(f); <- still won't compile as it has been inferred as an int
}

fn accept_closure_string(f: fn(String) -> String) {
    let x = f(String::from("hello"));
}

fn accept_closure_int(f: fn(u32) -> u32) {
    let x = f(3);
}

// we can create a struct which holds a closure, lazily evaluates the result, and
// caches it for any other code that needs it

// these structs need to specify the type of the closure, because the struct
// each closure has its own unique anon type, even if they have the same sig
// we use generic params and trait bounds to define structs, enums, or functions that use
// closure

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v); // cache
                v // return
            }
            Some(val) => *val
        }
    }
}


// the cacher has limits:  it will only have one value once the virst call to c.value has
// been made

fn capturing_environment() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
// when a closure captures a value from its env it uses memory to store the values for use
// in the closure body. This is overhead we don't want to pay in normal functions.
// Closures can capture env in 3 ways
/*
FnOnce
    consumes the variables from its enclosing scope, and takes ownership of the variable
    this variable. It can only capture it once, hence FnOnce

FnMut
    can change the environment because it mutably borrows values

Fn
    immutable borrow of environment
 */
// traits are infered by usage, going from FnOnce, Fn, FnMut

fn move_keyword() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z: Vec<i32>| z == x;
    // println!("cant' use x here {:?}", x); <- compile error
    // x has been moved and is owned by equal_to_x
}
