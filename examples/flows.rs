#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::collections::LinkedList;

fn main(){

    // for in iter
    let a: [i32; 5] = [0, 10, 20, 30, 40];

    for element in a.iter() {
        println!("Searching through array, currently on element: {}", element);
        if element == &20 {
            println!("Found element {}", element);
            break;
        }
    }

    let counter: i32;
    // for in range
    for number in (0..=3){
        println!("Current index: {}", a[number]);
    }

    let mut ll = LinkedList::new();
    ll.push_back(1);
    ll.push_back(2);

    for element in ll.iter() {
        if element == &1 {
            println!("Found item: {}", element);
        }
    }
}