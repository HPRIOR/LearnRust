// option is a  n enum defined by the standard library

// rust does not have null values

fn main() {
    let some_number = Some(5);
    let some_string = Some("some string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(8);
    let result = match y {
        None => x,
        Some(x) => y + x
    };
    // these cannot be added as they are
}