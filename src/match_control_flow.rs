use crate::match_control_flow::UsState::Alabama;

// pattern matching
// compare against a series of patterns and conditionally execute code depending on
// on the matched patter (conveyor belts!)
enum UsState {
    Alabama,
    Alaska,
    // so on
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self { // can return any value (unlike conditionals which return bools)
            Coin::Penny => 1, // braces can be used if you need statements
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => match state { // Because enums can hold
                UsState::Alabama => 20,                    // values, these can be
                UsState::Alaska => 25                      // by the match.
            }
        }
    }
}

fn using_value_match() {
    let value_of_dime = Coin::Dime.value_in_cents();
    let value_of_quarter_in_alabama = Coin::Quarter(Alabama).value_in_cents();
}


// matches are useful when handling options

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // matches must be exhaustive and account for every possibility
        Some(i) => Some(i + 1), // bind variable to data inside enum
    }
}

fn use_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


// we can use _ placeholder as default behaviour
fn match_u8(i: u8) {
    match i {
        1 => println!("{}", 1),
        _ => {}
    }
}

// if you want to match just one value you can use the if let syntax
// fn match_just_one(i: u8) {
//     if let Some(1) = i { println!("three") }
// }




