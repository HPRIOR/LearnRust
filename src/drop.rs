/*
Drop trait allows you to customise what happens when a value goes out of scope.
This can be implemented on any type (you can use it to control the release of resources
such as network connections and files)
 */
struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data {}!", self.data)
    }
}

fn use_custom_smart_pointer(){
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.")

} // will print drop messages here

// you cannot call drop manually
// we have to call std::med::drop

fn calling_drop_manually(){
    let c = CustomSmartPointer{
        data: String::from("Some data"),
    };
    println!("CustomSmartPointer created.");
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}


