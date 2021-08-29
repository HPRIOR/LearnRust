pub fn this_main() {
    another_function(5, 6); // snake case is convention
}

fn another_function(x: i32, y: i32) { // types must be annotated in the sig
    println!("Another function");
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
}

// function bodies can contain expressions and statements
fn statement() {
    let y = 6;
    // let x = (let y = 6); statements can't return values so this won't compile
}

fn expressions() {
    let x = 5;
    let y = { // creates new scope (and is an expression because returns value)
        let x = 3;
        x + 1 // expressions will return values which don't have a semi colon
        // x + 1; this would turn the scope into a statement which does not return anything
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 { // functions can be type annotated
    5 // implicit return
}
// fn not_five() -> i32 { // will throw an error
//     5;
// } // return empty tuple ()

fn trying_to_use_unit() {
    let x = five();
}





