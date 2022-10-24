fn main() {
    // integer scalar types
    let a: u32 = 12;
    println!("value of a: {}", a);

    let x: i32 = -1;
    println!("value of x: {}", x+2);

    // Floating scalar types
    let y: f32 = 321.123;
    println!("value of y: {}", y);

    //Simple numeric operations
    let a: f32 = a as f32;
    let z: f32 = a / a;
    println!("value of z: {}", z);

    // Boolean
    let happy: bool= false;
    let coding: bool = false;
    println!("it is {} that i'm hungry", happy && coding);

    // Char
    let c = '👺';
    println!("char value is {}", c);

    // Tuple (compound type)
    let point_3d:(i32, i32, i32) = (1, 10, 1);
    println!("value of point3D: {:?}", point_3d);

    let multiple_type: (i32, i32, i32, f32) = (1, 2, 3, 3.0);
    println!("value of multipleType: {:?}", multiple_type);
    println!("value of 3rd element in multipleType: {:?}", multiple_type.3);

    // Array (compound type)
    let array:[i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("content of array: {:?}", array);

    let seasons: [&str; 4] = ["Winter", "Spring", "Summer", "Autumn"];
    println!("seasons of the year: {:?}", seasons[0]);



}