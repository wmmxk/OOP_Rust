#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,

}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Operand(isize),

}

fn main() {
   let tokens = vec![Token::Operator(Operator::Add)];
   let ref_tokens = &tokens;

   for token in ref_tokens.iter() {
      match token {
      &Token::Operator(ref local_op) => {println!("1st element: {:?}",local_op);},
      _ => (),
      }    
   }

   for token in ref_tokens {
      match token {
      &Token::Operator(local_op) => {
             println!("1st element: {:?}",local_op);
             println!("conversion: {}",local_op as usize);
      },
      _ => (),
      }     
   }
}
