// this code is to show you can not use a reference when you destructure a variant,
// ref local means you declare a variable named local and local is a referece to something
// & local means get the address of the variable local
#[derive(Debug)]
enum Op {
     Add,
     Sub,
}

enum Token {
     Operand(usize),
     Operator(Op),
}

fn main(){

let op1 = Op::Add;
let t1 = Token::Operator(op1);
match t1 {
      Token::Operator(ref local_op) => println!("operator {:?}", local_op),
      Token::Operand(num) => println!("number {}", num),
}

match t1 {

      // not work Token::Operator(ref local_op) => println!("operator {:?}", (local_op as usize )),
      Token::Operator(local_op) => println!("operator {:?}", (local_op as usize )),
      Token::Operand(num) => println!("number {}", num),
}

}
