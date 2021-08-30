/*
Packages and crates

A crate is a binary or library. Its root is a source file that the compiler starts from
and makes up the root module of your crate.

A package contains one or more crates that provides a set of functionality. It contains
a Cargo.toml file that describes how to build those crates.

A package must contain zero or one library crate, although it it can contain many
binary crates as you'd like. It must contain at least one crate either bin or lib.

To create a new package we type cargo new project-name into the command line.
This creates a Cargo.toml file giving us a package. The convention is that src/main.rs
is the root of the project.

This package only contains src/main.rs, meaning that it only contains a binary crate
named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates,
a library and a binary, both with the same name as the package.

A package can have multiple bin crates by placing them in a src/bin directory.
Each file will be a separate binary crate

The crate will group related functionality together in a scope so that the
it is easy to share between multiple projects. We can use that functionality
by bring a crate into our project's scope, which is accessible through the crates name
*/

/*
Modules

Modules let us organise code within a crate into groups for readability and easy reuse
They also control privacy -  whether or not they can be used outside code, or are an
internal implementation detail

*/

// example library crate that provides functionality of a restaurant
// front of house, back of house