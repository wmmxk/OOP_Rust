// loop through a ref to an array, the data is not moved
// point 1: If you loop through the vector directly, you can not use it anymore.
// point 1: When there is a borrow, you can move the data.

fn main()
{
  let mut v1 = vec![String::from("hello"), String::from("world")];

  let v2: &[String];

//{
//  v2 = &v1; 
//  for t in v2 {
//  println!("each element: {:?}", t);
//}
//  for t in v2 {
//  println!("each element: {:?}", t);
//}
//
//}

  for t in v1 {
  println!("each element: {:?}", t);
}
  for t in v1 {
  println!("each element: {:?}", t);
}

}
