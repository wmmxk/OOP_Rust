#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
		pub fn can_hold (&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
		}

}

pub fn add_two(a:i32) -> i32 {

   a +2 
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
       let larger = Rectangle {length: 8, width:7};
       let smaller = Rectangle {length:5, width: 1};
       assert!(larger.can_hold(&smaller));
    }

    #[test]
	fn it_adds_two() {
       assert_eq!(4,add_two(2))

	}

}
