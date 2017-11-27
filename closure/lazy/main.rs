fn main(){
let mut c = 0;
for pair in vec!['a','b','c'].into_iter().map(|letter| {c+=1; (letter, c)}).rev(){
  println!("{:?}",pair);
}


}
