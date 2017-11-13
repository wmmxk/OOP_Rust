// source: https://www.reddit.com/r/rust/comments/46qwjv/why_can_i_use_an_mut_reference_twice/

fn increment(x: &mut i32)  {
   *x +=1;
}

fn main() {
   let mut x = 0; 
   let y = &mut x;
   increment(y);
   
   increment(y);
   println!("{}",y);

}
