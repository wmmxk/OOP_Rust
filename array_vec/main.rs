// &[T] can be a vector or an array
fn display<T:std::fmt::Debug>(vs: &[T]) {
 for v in vs {
 println!("{:?} ", v);
 }
}

fn main() {
let vs1 = &vec![1,2,3];
let vs2 = &[5,6];

display(vs1);
display(vs2);
 
}
