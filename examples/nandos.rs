use std::convert::{TryFrom, TryInto};
use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Main {
    Chicken,
    Burger,
    Lamb
}


impl TryFrom<String> for Main {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Burger" => { Ok(Main::Burger) }
            "Chicken" => { Ok(Main::Chicken) }
            "Lamb" => { Ok(Main::Lamb) }
            _ => {Err(())}
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum Sides {
    Rice,
    Fries,
    Coleslaw
}

impl TryFrom<String> for Sides {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Rice" => { Ok(Sides::Rice) }
            "Fries" => { Ok(Sides::Fries) }
            "Coleslaw" => { Ok(Sides::Coleslaw) }
            _ => {Err(())}
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Drinks {
    Fanta,
    Coke,
    Sprite
}

impl TryFrom<String> for Drinks {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {

            "Fanta" => { Ok(Drinks::Fanta) }
            "Coke" => { Ok(Drinks::Coke) }
            "Sprite" => { Ok(Drinks::Sprite) }
            _ => {Err(())}
        }
    }
}

struct NandosOrder {
    main: String,
    side: String,
    drink: String
}

fn main() {
    println!("What is your name ? ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read order");
    println!("Hi {}, what will you like to order from Nandos", name.trim());

    let mut main = String::new();
    io::stdin()
        .read_line(&mut main)
        .expect("Failed to read order");
    println!("You have selected >{}< for your main", main.trim());

    println!("What sides would you like with the >{}< ?", main.trim());
    let mut side = String::new();
    io::stdin()
        .read_line(&mut side)
        .expect("Failed to read order");
    println!("You have selected >{}< for your sides", side.trim());


    println!("What drink would you like ?");

    let mut drink = String::new();
    io::stdin()
        .read_line(&mut drink)
        .expect("Failed to read order");
    println!("You have selected >{}< for drink", drink.trim());

    let order = NandosOrder(main, side, drink);
    println!("Your order is {}", order);



}


