//source: https://www.reddit.com/r/rust/comments/46qwjv/why_can_i_use_an_mut_reference_twice/
fn f1(x: &mut i32) -> &mut i32 {
   *x +=1;
   x
}

fn main() {
   let mut x =0;
   let y = &mut x;
   let z=   f1(y);
   //   f1(y);
   println!("{}",y);


}
