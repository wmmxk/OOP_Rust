#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

fn validate(cur: &InfixToken, prev: Option<&InfixToken>) -> bool {
    match *cur {
        InfixToken::Operand(_) | InfixToken::LeftParen =>
            prev.is_none() || match *prev.unwrap() {
                InfixToken::LeftParen | InfixToken::Operator(_) => true,
                InfixToken::RightParen | InfixToken::Operand(_) => false,
            },
        InfixToken::Operator(_) | InfixToken::RightParen =>
            prev.is_some() && match *prev.unwrap() {
                InfixToken::RightParen | InfixToken::Operand(_) => true,
                InfixToken::LeftParen | InfixToken::Operator(_) => false,
            },
    }
}           

fn main() {
    let tokens_v = vec![InfixToken::Operand(5), InfixToken::Operator(Operator::Add),InfixToken::Operand(4)];
    let tokens:&[InfixToken];
    tokens = & tokens_v;
    enum StackToken {
        Operator(Operator),
        LeftParen,
    }

    let precedence = [1, 1, 2, 2];
    let mut stack = vec![];
    let mut output = vec![];
    for token in tokens {
        match token {
            &InfixToken::Operand(operand) => output.push(PostfixToken::Operand(operand)),
            &InfixToken::LeftParen => stack.push(StackToken::LeftParen),
            &InfixToken::RightParen => {
                loop {
                    match stack.pop() {
                        Some(StackToken::Operator(operator)) => {
                            output.push(PostfixToken::Operator(operator))
                        }
                        Some(StackToken::LeftParen) => break,
                        None => (),
                    }
                }
            }
            &InfixToken::Operator(operator) => {
                while !stack.is_empty() {
                    match stack[stack.len() - 1] {
                        StackToken::LeftParen => break,
                        StackToken::Operator(op2) => {
                            if precedence[op2 as usize] >= precedence[operator as usize] {
                                match stack.pop().unwrap() {
                                    StackToken::Operator(op) => {
                                        output.push(PostfixToken::Operator(op))
                                    }
                                    StackToken::LeftParen => panic!(),
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                stack.push(StackToken::Operator(operator));
            }
        }
       }
    while let Some(v) = stack.pop() {
        match v {
            StackToken::Operator(v) => output.push(PostfixToken::Operator(v)),
            _  => (),
        }
    }
    println!("{:?}", output);
}

