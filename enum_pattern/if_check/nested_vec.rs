#[derive(PartialEq,Debug)] 
enum Infix {
    Operand(usize),
    Operator,
}


fn main() {

    let tokens = vec![Infix::Operand(4), Infix::Operand(5)];
  
	match tokens[0] {

			Infix::Operand(num1) => {
//                if let Infix::Operand(num2) = tokens[1] {
//                if tokens[1] ==  Infix::Operand(num2) { // this case does not work
                if tokens[1] ==  Infix::Operand(5) {
                     println!("Second number {}", 5);
                }
			println!("First number {} ", num1);
            },
        
    _ => println!("Others"),

    }    
}
