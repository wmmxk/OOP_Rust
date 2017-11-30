//source: https://stackoverflow.com/questions/27305585/difference-between-pass-by-reference-and-by-box
fn f1(x: &mut i32) {
  *x =5;
}

fn f2(mut x: Box<i32>) {
   *x =5;
}

fn main() {

   let mut s_a = 3;
   let mut h_a = Box::new(4);

   f1(&mut s_a);
   println!("{}", s_a);
   
   f1(&mut s_a); 
  
   f2(h_a);
//   println!("{}", h_a);

}
