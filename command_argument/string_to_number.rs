//source: https://doc.rust-lang.org/book/first-edition/error-handling.html
// store the first argument in a String variable and then parse the String
// as a number
use std::env;

fn main() {
   let mut argv = env::args();
   let arg: String = argv.nth(1).unwrap();
   let n: i32 = arg.parse().unwrap();
   println!("{}",2*n);


}
