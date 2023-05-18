/// Returns true if the input is `Ok(x)` of some even `x`.
pub fn match_4(input: Result<u32, &'static str>) -> bool {
    // let check: i32 = match input.unwrap() { Ok(v) => v, Err(_) => -1 };
    // if check >=0 && number % 2 == 0 {
    //     return true;
    // }
    // return false;

    match input {
        Ok(n) => { return n % 2 == 0 }
        Err(_) => { return false }
    }
}


fn main() {
    println!("{}", match_4(Ok("12".parse().unwrap())));
}