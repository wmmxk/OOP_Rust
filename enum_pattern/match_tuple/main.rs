#[derive(Debug)]                                                                                              
enum Operator {                                                                                           
    Add,                                                                                                      
    Sub,                                                                                                      
    Mul,                                                                                                      
 }                                                                                                             
                                                                                                              
#[derive(Debug)]                                                                                              
enum Token {                                                                                              
    Operator(Operator),                                                                                       
    Operand(isize),                                                                                           
 }                                                                                                             


fn judge(pre: &Token, cur: &Token) -> bool {
		match (pre, cur) {
             (&Token::Operator(ref op1),&Token::Operator(ref op2)) => false, 
             
             (&Token::Operand(n1),&Token::Operand(n2)) => false, 
             _ => true,

		}
}

fn main()
{
   let   t1 = Token::Operand(2);
   let   op2 = Operator::Add;
   let t2 = Token::Operator(op2);

   let   res = judge(&t2, &t2);
   let   res2 = judge(&t1, &t1);
   println!("result: {}",res);

   println!("result: {}",res2);

}
