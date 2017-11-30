// This is to show how to handle edge case
#[derive(Debug)]
enum MathError {
   DivisionByZero,
   Others,
}
type MathResult = Result<f64, MathError>;
fn div (x: f64, y: f64) -> MathResult {
		if y==0.0 {
             Err(MathError::DivisionByZero)
		} else {
             Ok(x/y)
		}
}
fn div2 (x: f64, y: f64) -> f64 {
             x/y
}
fn main(){
let x=2.2;
let y=0.0;
println!("{}",div(x,y).unwrap());
println!("{}",div2(x,y));
		
}
