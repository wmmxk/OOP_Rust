//This is to show the difference of ref and &
fn main() {
  let x = 12;

  let s1 = Some(&x);

  let s2 = Some(ref x);


}
