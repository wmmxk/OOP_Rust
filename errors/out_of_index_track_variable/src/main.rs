//source: https://doc.rust-lang.org/book/second-edition/ch09-01-unrecoverable-errors-with-panic.html


// how to backtrace the error:
// RUST_BACKTRACE=1 cargo run  . it wil trace to the line in the main function

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}
impl<T:std::fmt::Debug>  Matrix<T> {
    fn display(&self) {
		for i in 0..self.row*self.col {
           println!("{:?} ", self.data[i]);
		}
    }      
}
fn main() {
		let  m1 = Matrix{
             data:vec![1,2,3],
             row: 3,
             col: 2,             
		};
        println!("{}",m1.data[1]);
        m1.display();
}
