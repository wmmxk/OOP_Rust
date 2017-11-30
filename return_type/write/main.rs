use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T: ops::Add<Output = T>> ops::Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T: ops::Add<Output=T> + ops::Sub<Output=T> + ops::Mul<Output=T> + Copy>
    ops::Mul for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex{ re: self.re * rhs.re - self.im * rhs.im,
                 im: self.re * rhs.im + self.im * rhs.re }
    }
}

impl<T: fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 1..2 {
          write!(f, "{}j", i)
        }
    }
}

fn main() {
   let x = Complex{re:1, im:2 };
   println!("{:?}",x);

}
