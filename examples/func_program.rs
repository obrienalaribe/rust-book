
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let y = 1;
    let plus_one = numbers.iter().map(|x| x + y)
        .for_each(|x| println!("{}", x));

    println!("------");
    numbers.into_iter().filter(|&x| x > 2)
        .for_each(|x| println!("{}", x));
    println!("------");
    let number2 = vec![10, 2, 3, 4, 5];
    let sum:i32 = number2.iter().sum();
    println!("sum of numbers: {}", sum);

    let max: &i32 = number2.iter().max().unwrap();
    println!("Max: {}", max);


}