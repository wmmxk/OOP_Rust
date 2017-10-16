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
    let mut stack = Vec::new();
    
    for token in tokens.iter() {
        match token {
            &Token::Operand(x) => stack.push(x),
            &Token::Operator(ref operator) => {
                let op1;
                let op2;
                match stack.pop() {
                    Some(operand) => op2 = operand,
                    None => return None,
                }
                match stack.pop() {
                    Some(operand) => op1 = operand,
                    None => return None,
                }
                let result = 
                    match *operator {
                        Operator::Add => op1 + op2,
                        Operator::Sub => op1 - op2,
                        Operator::Mul => op1 * op2,
                    };
                stack.push(result);
            },
        }
    }

    
    match stack.pop() {
        Some(v) => {
            if stack.is_empty() {
                Some(v)
            } else {
                None
            }
        },
        None => None
    }
}



#[cfg(test)]    
mod tests {    
        use super::*;    
        #[test]    
        fn test_Token() {    
        let  expression  = vec![Token::Operand(3), Token::Operand(4), Token::Operator(Operator::Add)];       
        println!("token 1:  {:?}\n",expression[0]);    
        println!("token 3: {:?}\n", expression[2]);    
        }  

        #[test]
        fn test_add() {
        let  expression  = vec![Token::Operand(3), Token::Operand(4), Token::Operator(Operator::Add)];       
        let res = eval(&expression);
        assert_eq!(7,res.unwrap());

        }

 
} 
