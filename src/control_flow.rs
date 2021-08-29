pub fn this_main() {
    let number = 3;

    if number < 3 { // if expression
        println!("Number is less than 3")
    } else if number > 5 { // else expression
        println!("Number is greater than 5")
    } else {
        println!("Number is greater less than or equal to 5 and greater than or equal to 3")
    }
    // values are not truthy, you cannot do 'if number'

    // if is an expression so the result can be bound to a variable

    let condition = true;
    let number = if condition { 5 } else { 6 }; // types must be compatible

    // Loops

    // loop keyword will continue to loops until explicitly told to stop
    let mut tick = 0;
    loop {
        tick += 1;
        print!("{}", tick);
        if tick == 10 { break; }
    }
    // loop results can be captured
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // result of loop expression
        }
    };
    println!(" The result is {}", result);

    // While loops
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("BLASTOFF!");

    // for loops

    let a = [1; 5];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 { // numbers could be reversed with (1..4).rev()
        println!("{}", number);
    }
}

