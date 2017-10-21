//source: https://stackoverflow.com/questions/15619320/how-to-access-command-line-parameters

// two ways to extract the parameter value passed from command: 
   // 1. you can convert the iterator to a vector using collect method first, then index element
   //  2. use the method of iterator
use std::env;

fn main() {

  let args: Vec<_> = env::args().collect();
  if args.len() > 1 {

     println!("The first argument is {}", args[1]);
  }

}
