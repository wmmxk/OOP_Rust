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

pub fn eval(tokens: &[Token]) -> Option<isize> {
}



#[cfg(test)]    
mod tests {    
        use super::*;    

        #[test]
        fn test_add() {
        let  expression  = vec![Token::Operand(3), Token::Operand(4), Token::Operator(Operator::Add)];       
        let res = eval(&expression);
        assert_eq!(7,res.unwrap());

        }

 
} 
