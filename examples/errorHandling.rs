use std::io::Error;

fn first_element(list: Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    } else {
        return Some(list[0]);
    }
}

fn connect_to_network(host: &str) -> Result<&str, &str>{
    if host == "localhost" {
        return Ok("Success");
    } else {
        return Err("Failed");
    }
}
fn main() {
    let list = vec![1, 2, 3, 4];

    match first_element(list) {
        None =>     println!("Empty list"),
        Some(x) => println!("first element: {}", x)
    }

    match first_element(Vec::new()){
        None => println!("Empty List"),
        Some(x) => println!("first element: {}", x)
    }
    println!("Making network connection {:?}", connect_to_network("remote"));
}