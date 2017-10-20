// This is code is show, the .iter() function borrows each element in the vector.
// cannnot move out of borrowed content. if don't use ref keyword in the first match arm, you will see such an error

// For integer type, when you bind it to another variable, you make a copy



#[derive(Debug)]
pub enum Aoperator {
  Add,
  Sub,
  Mul,
}

#[derive(Debug)]
pub enum Token {
   Operator(Aoperator),
   Operand(isize),
}

fn main()

{
   let tokens = vec![Token::Operator(Aoperator::Add)];
  
   println!("a token: {:?}", tokens); 
   for token in tokens.iter() {
		   match token {
			 &Token::Operator(ref localv) => println!("the included operator {:?}", localv),
			 &Token::Operand(num) => println!("The number included {}", num),

		   }
   }
}
