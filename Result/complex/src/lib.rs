// This is to show the write function returns a result type

use std::{fmt};

#[derive(PartialEq, Debug)]
pub struct Complex<T> {
  re: T,
  im: T,
}


impl<T: fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = 0;
	    for i in 1..3 {	
				if count < 2 {
				write!(f, "{}{:+}j", self.re, self.im)
				} else { 
				write!(f, "{}{:+}j", self.re, self.im)
				}
        }   
    }
}

#[test]
fn test2() {
     assert_eq!(format!("{}", Complex{ re: 1, im: -2 }), "1-2j");
     assert_eq!(format!("{}", Complex{ re: -1, im: 2 }), "-1+2j");
}

