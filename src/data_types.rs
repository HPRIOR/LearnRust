pub fn this_main() {

    // this will throw an error if not annotated with type
    let guess: u32 = "42".parse().expect("Not a number");

    // scalar types (singe value)

    // Integer types
    // can be signed (i) or unsigned (u)
    let eight_bit_unsigned: u8 = 1;
    let eight_bit_signed: i8 = 1;

    // integer overflow occurs when trying to change a variable
    // to that above its capacity an integer overflow will occur


    // Floating points
    // two types f32 and f64

    let f_64 = 2.0; // default type
    let f_32: f32 = 2.0; // has to be ype annotated

    // Numeric operations

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 42 % 5;

    // Boolean type
    let t = true;
    let f = false;

    // Character type
    let c = 'z';
    let z = 'ℤ';
    let hear_eyed_cat = '😻';

    // Compound types

    // Tuple type
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    // can be destructured
    let (x, y, z) = tup;
    // tuples can be accessed through their indexes
    let five_hundred = tup.0;

    // Array type
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // type annotated
    let c = [1; 5]; // syntax sugar for creating [1, 1, 1, 1, 1]

    // arrays are accessed through index
    a[1];

    a[10]; // will cause a panic and a runtime error
}