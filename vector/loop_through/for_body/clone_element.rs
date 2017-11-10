//This is to show a borrowed type can not be moved.
// If the element in the vector implements the copy trait. It is OK.
// Or use v2.push((*v).clone())
fn main(){
  let  v1 = vec![String::from("s1"), String::from("s2")];
  let mut v2 = vec![String::from("s3"), String::from("s4")];

{
  for v in &v1 {
//  v2.push(*v);
  v2.push((*v).clone());

  }
}
 println!("{:?}", v2[0]);

}


