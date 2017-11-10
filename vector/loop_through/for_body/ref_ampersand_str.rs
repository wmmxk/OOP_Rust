// This is to show you get a reference when you loop through a vector by a reference
fn main(){
  let  v1 = vec![String::from("s1"), String::from("s2")];
  let mut v2 = vec![String::from("s3"), String::from("s4")];

  for ref v in v1 {
//  v2.push(v);
 v2.push((*v).clone());   //correct
  }
 println!("{:?}", v2[0]);

}


