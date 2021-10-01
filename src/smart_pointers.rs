/*
Pointer is a general concept for a variable that contains an address in memory.
The most common kind of pointer in rust is a reference (&type). They only point to data,
and have no overhead.

Smart pointers are data structures that act like pointers, but also have additional
capabilities. In Rust different smart pointers are defined in the standard library,
providing extra functionality that mere pointers.

e.g. the reference counting smart pointer type - this enables you to have multiple owners
of data by keeping track of the number of owners, and when no owners remain, cleaning up
the data.

In Rust, smart pointers and references also differ in that references point to borrowed data,
and smart pointers can 'own' the data they point to.

Smart pointers are often implemented using structs. These are differentiated from normal
structs, as they often implement the Deref and Drop traits.

Deref allows an instance of teh smart pointer to behave like a reference, so you can write
code that works with either references or SPs.

Drop allows you to customize the code that is run when an instance of a smart pointer goes
out of scope.

You can write your own SPs, and many libraries provide their own. The most common ones will
be covered here:
Box<T>, allocate value on heap

Rc<T>, reference counting type enabling multiple owners

Ref<T> and RefMut<T>, accessed through RefCell<T>, enforces the borrowing rules at runtime
instead of compile time
 */