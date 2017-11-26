fn main() {
   let mut stack =  vec![];
   let opt1 = Some(3);
   opt1.map(|node| stack.push(node));
   println!("{:?}",stack);
   println!("{:?}",opt1);
}
