#![allow(unused_imports, dead_code, unused_variables, unused_assignments)]

use crate::variables_and_mutability::this_main as v_and_m_main;
use crate::data_types::this_main as data_types_main;
use crate::functions::this_main as functions_main;
use crate::control_flow::this_main as control_flow_main;
use crate::fib::fib;
use crate::ownership::this_main as ownership_main;
use crate::slice::this_main as slice_main;
use crate::structs::this_main as structs_main;
use crate::vectors::this_main as vectors_main;
use crate::iterators::manually_call_iter;

mod variables_and_mutability;
mod data_types;
mod functions;
mod control_flow;
mod fib;
mod ownership;
mod slice;
mod structs;
mod struct_example;
mod enums;
mod options;
mod match_control_flow;
mod module_system;
mod vectors;
mod strings;
mod hash_maps;
mod panics;
mod recoverable_errors;
mod generics;
mod traits;
mod lifetimes;
mod testing;
mod closures;
mod iterators;
mod cargo;
mod smart_pointers;
mod r#box;
mod deref;
mod drop;
mod rc_reference_counted;

fn main() {
    // v_and_m_main();
    // data_types_main();
    // functions_main();
    // control_flow_main();
    // print!("{}", fib(100, &mut HashMap::new()));
    // ownership_main();
    // slice_main();
    // structs_main();
    // vectors_main();
    manually_call_iter();
}
