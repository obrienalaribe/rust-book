macro_rules! hello {
    ("ob") => { println!("Hello OBrien!!")};
    ("nj") => { println!("Hello Nguya")};
    () => { println!("Hello Mr Nobody")};
}

fn main(){
    hello!("ob");
}