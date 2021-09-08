pub fn this_main(){
    // unrecoverable errors are handled by the panic! macro
    // this will print a failure message, unwind and clean up the stack, and then quit.
    // commonly occurs when a bug is found and it's not clear how to handle the error

    let v = vec! [1,2,3];
    v[99]; // will cause a panic
    panic!("shieeeeeeeeeeeeeeeeeeeeeet");

}