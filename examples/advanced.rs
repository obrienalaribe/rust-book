trait Animal {
    fn baby_name() -> String;
}

trait Mammal {
    fn legs() -> String;
}

struct Dog;
struct Human;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Mammal for Dog {
    fn legs() -> String {
        String::from("4")
    }
}

impl Mammal for Human {
    fn legs() -> String {
        String::from("2")
    }
}

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A dog has {} legs", <Dog as Mammal>::legs());
    println!("A human has {} legs", <Human as Mammal>::legs());
}
