//this is to show if you loop through element in the vector directly, the vector will be consumed by for loop;
// To demonstrate the point, you can use a vector of String
// you can not use the vectors, because push move the data or the for loop move the data
// println! is OK


#[derive(Debug)]
pub enum Operator {
  Add,
  Sub,
}

fn main()
{
  let  tokens = vec![Operator::Add, Operator::Sub];
  let mut output = Vec::new();

  
  for t in tokens.iter() {

   println!("token {:?}", t);
   output.push(t);
  }
 
 
  for t in output {

   println!("token {:?}", t);
  }
  
// you can not use tokens anymore
for t in tokens {
   println!("token {:?}", t);
  }

}
