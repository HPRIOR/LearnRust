const GLOBAL_CONSTANT: u8 = 5;

// constants can be declared in the global scope
pub fn this_main() {
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6; this will not compile because x is immutable
    // println!("The value of x is {}", x);

    let mut y = 5; // declaring a mutable variable
    y = 1; // the value can now change

    const CONSTANT: u8 = 1; // constants must have their type annotated
    // they can only be set only to a constant expression, not anything that can be
    // computed at runtime

    const MAX_POINTS: u32 = 100_000; // underscores can be added for clarity to any number

    let x = 6; // shadowing is possible on immutable variables
    // because we are effectively changing the variable, we can assign a new type:
    let x = "six";

    // shadowing allows us to modify a variable 'in-place'
    let spaces = "   ";
    let spaces = spaces.len(); // the types don't clash because of shadowing
    // this would not be possible with mutable variables
    // let mut spaces_again = "    ";
    // spaces_again = spaces_again.len(); will not compile;

    // mutable variables can mutate their values, not their types!
}