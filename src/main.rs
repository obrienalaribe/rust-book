macro_rules! blockchain {
    ("ob") => { println!("Hello OBrien!!")};
    ("nj") => { println!("Hello Nguya")};
    ("paidchain") => { println!("Hello Paidchain blockchain")};
    () => { println!("Hello Mr Nobody")};
}

fn main(){
    blockchain!("paidchain");
}