// This is to show you get a reference when you loop through a vector by a reference
fn main(){
  let  v1 = vec![1,2,3];
  let mut v2 = vec![4,5,6];

  for ref v in v1 { // wrong way: for &v in v1
      v2.push(*v);
  }
 println!("{:?}", v2[0]);

}


