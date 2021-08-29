use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub fn fib(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    match n {
        1 | 2 => 1,
        _ => {
            if memo.contains_key(&n) {
                return memo[&n];
            } else {
                let result = fib(n - 1, memo) + fib(n - 2, memo);
                memo.insert(n, result);
                result
            }
        }
    }
}