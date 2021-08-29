#![allow(unused_imports, dead_code, unused_variables, unused_assignments)]

use crate::variables_and_mutability::this_main as v_and_m_main;
use crate::data_types::this_main as data_types_main;
use crate::functions::this_main as functions_main;
use crate::control_flow::this_main as control_flow_main;
use crate::fib::fib;
use crate::ownership::this_main as ownership_main;
use crate::slice::this_main as slice_main;

use std::collections::HashMap;

mod variables_and_mutability;
mod data_types;
mod functions;
mod control_flow;
mod fib;
mod ownership;
mod slice;
mod structs;

fn main() {
    // v_and_m_main();
    // data_types_main();
    // functions_main();
    // control_flow_main();
    // print!("{}", fib(100, &mut HashMap::new()));
    // ownership_main();
    slice_main();
}
