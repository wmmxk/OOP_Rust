//source: https://doc.rust-lang.org/book/second-edition/ch10-01-syntax.html
// In your case, & or ref, either is fine. But in the source code, you must use &;
fn main() {
 
  let strs = vec![String::from("s1"), String::from("s2")];

  for ref str in strs.iter() {
    println!("each string {:?} :", str);
  }
  
  let nums = vec![1,2,3];
  let ref_nums = & nums;
  for &num in ref_nums.iter() {
     println!("Each number: {} ", num);
  }

}
