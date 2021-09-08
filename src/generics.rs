// non generic function
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic version
fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic point - x,y must be of same type
struct Point<T> {
    x: T,
    y: T,
}

// methods can be implemented on structs and enums with generic types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we can still implement methods on non - generic points
// only points with this type will have this method as well as any generic methods
impl Point<f32> {
    fn y(&self) -> &f32 {
        &self.y
    }
}

// can be of two data types
struct PointTwoDataTypes<T, U> {
    x: T,
    y: U,
}

// generic type parameters in struct definition aren't always the same as used in method
// sig. This takes two points with diff sigs and return a combined sig
impl<T, U> PointTwoDataTypes<T, U> {
    fn mixup<V, W>(self, other: PointTwoDataTypes<V, W>) -> PointTwoDataTypes<T, W> {
        PointTwoDataTypes {
            x: self.x,
            y: other.y,
        }
    }
}

// generic enums
enum Option<T> {
    Some(T),
    None,
}

// with two types
enum Result<T, E> {
    Ok(T),
    Err(E),
}
