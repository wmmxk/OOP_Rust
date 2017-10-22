//This code is to confirm you can not move the data if there is a reference 
// pointing to the data.
// In specific, in the for loop, you can loop through the tokens directly.
#[derive(Debug)]
pub enum Operator {
  Add,
  Sub,
}

fn main()
{
  let  tokens = vec![Operator::Add, Operator::Sub];
  let ref_tokens: &[Operator] = & tokens;
  for t in ref_tokens {

   println!("token {:?}", t);
  }
  

}
