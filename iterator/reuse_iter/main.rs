fn main(){
  let v1 = vec![1,2,3];

  let v1_iter = v1.iter();
  let total : i32 = v1_iter.sum();
  println!("sum is {}", total);

  let total2 : i32 = v1_iter.sum();

}
