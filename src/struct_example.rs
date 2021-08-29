fn a_simple_program() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of a rectangle is {} square pixels", area(width1, height1));
}

// initial area function
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// we should make it more readable, as the parameters are related they should be grouped

fn tuple_refactoring() {
    let rect1 = (30, 50);
    println!("The area of a rectangle is {} square pixels", area_with_tuple(rect1));
}

// dimensions are grouped but their meaning is obscured by just calling their index
fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Struct refactoring
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// derive debug implements the debug trait on struct, allowing us to use {:?} or {:#?} in string
// interpolation

fn struct_refactoring() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of a rectangle is {} square pixels", area_with_rect(&rect1));
    println!("The area of a rectangle is {} square pixels", rect1.area());
}

fn area_with_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Traits (interfaces)
// to add printing functionality to rectangle struct we can implement the Display or Debug trait


impl Rectangle {
    // contains methods for Rectangle
    fn area(&self) -> u32 { // methods can take ownership/borrow of self immutably/mutably
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.height < self.height && other.width < self.width
    }
    // we can also define static or associated functions
    // this is referenced using ::
    // these are often used for constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn with(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
// in some language you are required to use . or -> depending on whether or not a method
// is being called directory on an object or through a reference
// if object is a pointer, object->something == (*object).something()


// structs allow you to have multiple impl blocks, which are useful in certain situations
// discussed in later chapters 

