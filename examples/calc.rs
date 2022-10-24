use std::collections::HashMap;
mod calc_mod;
use calc_mod::calculator::{fib_dyn};

fn main(){
    let mut map = HashMap::new();

    for i in 1..100 {
        println!("{}: {}", i, fib_dyn(i, &mut map));
    }
}