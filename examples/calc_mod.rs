#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::collections::HashMap;

/// This is a doc comment for my calculator!
///
/// ```rust
/// calculator::fib()
/// ```
pub mod calculator {
    use std::collections::HashMap;

    const FIB_ZERO: u64 = 1;
    const FIB_ONE: u64 = 1;

    pub fn fib(n: u64) -> u64 {
        if n == 0 {
            FIB_ZERO
        } else if n == 1 {
            FIB_ONE
        } else {
            fib(n-1) + fib(n-2)
        }
    }

    pub fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
        match n {
            0 | 1 => 1,
            n => {
                if map.contains_key(&n) {
                    *map.get(&n).unwrap()
                } else {
                    let val = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                    map.insert(n, val);
                    val
                }
            }
        }
    }

}

fn main() {}