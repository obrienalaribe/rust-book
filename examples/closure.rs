fn main() {
    let val = 10;
    let closure1 = |x| { x + val};
    println!("{}", closure1(1));
}