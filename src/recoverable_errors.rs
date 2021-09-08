use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{io, fs};

pub fn this_main() {
    // most errors aren't serious enough that they require the program to stop entirely
    // we need some way to handle these appropriately

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // complex pattern matching example
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file, // return file if found
        Err(error) => match error.kind() { // match over errors if not
            ErrorKind::NotFound => { // if not found then create file
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("{:?}", e)
                }
            }
            other_error => { // with other errors just panic
                panic!("problem ")
            }
        }
    };

    // shorthand - will panic if no file is found
    let f = File::open("hello.txt").unwrap();
    // a similar method expect lets us choose the panic error message
    let f = File::open("hello.txt").expect("this didn't work");

    // errors can be propogated by returning an error in the Result type

}

fn maybe_do_something(yes: bool) -> Result<u32, String> {
    if yes {
        Ok(42)
    } else {
        Err("this shit don't work".to_string())
    }
}

fn read_username_from_fil() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// shortcut for propagating errors
fn read_username_shorthand() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;   // if this produces an error
    let mut s = String::new();                  // then the it will return
    f.read_to_string(&mut s)?;                     // out of the function with an
    Ok(s)                                               // io error
}

fn even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn even_even_shorter() ->  Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

// the ? can be used on functions that return a result type
// e.g. it cannot be used in main because main has a return type of ()
// any funciton with a  return type that implements Try, can use the ?
// Option, Result etc
