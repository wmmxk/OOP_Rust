use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Complex<T> {
    re: T,
    im: T,
}

fn main() {
  let c1 = Complex { re:1, im:2};

  let mut c2 = (c1).clone();
  c2


}
