#![allow(unused)] // silence unused warnings while exploring (to comment out)
fn main() {
    // variables
    let mut x = 280;
    println!("value of x: {}", x);

    // Constants
    const Y: i32 = 222;
    println!("value of Y {}", Y * 2);

    // variable shadowing
    let a = "123";
    let a: i32 = a.parse().unwrap();
    let result  = a * 3;
    println!("Value of a: {}", result);
}