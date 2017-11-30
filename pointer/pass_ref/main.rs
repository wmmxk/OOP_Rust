fn succ(x: &isize) -> isize {*x +1}

fn main() {
  let num = 5;
  let succ_num = succ(num);
  println!("{}", succ_num);


}
