struct MasterCard{
    number: u8,
    verification: u8,
}
struct Visa {
    number:u32,
}
struct WesterUnion{
    name:String,
    verification:u8,
}

struct BitCredit<T>{
    pub btnnumber: T,
}
impl BitCredit{
    fn test(&self){

    }
}

impl Into<BitCredit> for u8 {
    fn into(self) -> BitCredit {
        BitCredit { btnnumber: self as u32 }
    }
}

trait CreditCharge {
    fn charge_with_id(&self, id:u32) -> bool;
}

impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btnnumber % 2
    }
}

fn transact<Q: CreditCharge>(issuer: Q){
    // get verif code from user
    let id = 4096;
    if issuer.charge_with_id(id) {
        println!("Success");
    } else {
        panic!("Invalid code");
    }
}

fn main(){
    let bitcredit = BitCredit { btnnumber: 1124 };
    bitcredit.test();
    let visa = Visa { number: 1124 };

    let code = 4096;
    transact( bitcredit);
}