
fn main() {
    let mut values = vec![1, 2, 3, 4, 5000, 6];
    let sum = borrow_sum( &values);
    // let x =  values;

    println!("sum of {} values is {} ", values.len(), sum);
    cap_values_owned(2, &mut values);
    println!("capped {:?}", values);
    println!("values {}", values.len());

    let a = vec![10, 7, 5, 3];
    let sla = &a[..2];
    println!("sla is {:?}", sla);

    let b = String::from("Hello");
    let slb = &b[0..2];
    println!("slb: {}", slb);

}

fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value;
    }
    return sum;
}
fn borrow_sum(v: &Vec<i32>) -> i32{
    let mut sum = 0;
    for value in v {
        sum += *value;
    }
    return sum;
}

fn cap_values_owned(max: i32, v: &mut Vec<i32>){
    for index in 0..v.len(){
        if v[index] > max {
            v[index] = max;
        }
    }
}