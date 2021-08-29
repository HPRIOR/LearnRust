pub fn this_main() {
    // structs let you name and package together multiple related values into a meaningful
    // group - similar to a object/class in other languages (records in F#)

    // similar to tuples, except types are named, you don't index into struct but refer
    // to name of fields
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // to instantiate a struct we specify values for each field

    let mut user1 = User {
        email: String::from("someexample@email.com"),
        username: String::from("user"),
        active: true,
        sign_in_count: 1,
    };

    // to access fields we use the dot notation
    user1.email = String::from("someotherexample@email.com");
    // to modify a field the whole struct must be mutable - you can't make single
    // fields mutable

    // function that returns an instance of a user
    fn build_user(email: String, username: String) -> User {
        User {
            email, // can use shorthand if parameters and field names are the same
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // struct update syntax
    let user2 = User {
        email: String::from("someexample@email.com"),
        username: String::from("user"),
        ..user1 // get the remaining fields from the another object
    };

    // tuple structs without named fields

    // these look similar to structs but don't have names associated with their values
    // just types

    // this is useful when you want to give the whole duple a name and differentiate it
    // from other tuples with the same values (to be its own type)

    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    // these are different types
    let black = Colour(0, 0,0);
    let origin = Point(0 ,0, 0);

    // they behave like tuples (destructuring, dot index)
    let x = origin.0;

    // unit struct represents no data

    let nothing = ();

    // Struct data ownership

    // we deliberately used String rather than &str because we need instances to own their
    // own data, and for that data to be valid for the life of the struct
    // structs can store data belonging to something else, however you need to use lifetimes

    // Example program
    let width1 = 30;
    let height1 = 50;

    println!("The area of a rectangle is {} square pixels", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32{
    width * height
}