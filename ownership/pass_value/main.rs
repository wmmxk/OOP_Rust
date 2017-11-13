// This is a contrived example to repeat the error in HW3.
// When you run *self + *rhs, you are moving the data, because pass by value
pub struct Complex {
  re: usize,
  im: usize,
}

fn display(com: Complex) {
  println!("the real part is {} ",com.re);
}

fn main() {
  let  com = Complex{ re:1, im:2 };
   display((*(&com)));


}
