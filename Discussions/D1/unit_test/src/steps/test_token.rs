#[derive(Debug)]
pub enum Operator {
   Add,
   Sub,
   Mul,
}


#[derive(Debug)]
pub enum Token {
    GOperator(Operator),
    Operand(isize),

}


// This annotation on the test module tells Rust to compile and run the test code only when we run cargo test
#[cfg(test)]
mod tests {
		use super::*;
       // A function marked as a unit test
		#[test]
		fn test_Token() {
		let  expression  = vec![Token::Operand(3), Token::Operand(4), Token::GOperator(Operator::Add)];
		println!("token 1:  {:?}\n",expression[0]); 
		println!("token 3: {:?}\n", expression[2]);
		}
}
