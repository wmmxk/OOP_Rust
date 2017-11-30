// don't how to initialize a Formatter type
use std::{ops, fmt};
struct Point {
  x:isize,
  y: isize
}
fn main(){

let f :&mut fmt::Formatter;

let p1 = Point {x:1,y:2};
for i in 1..3 {
   let a =2;
   if i <2 {
   write!(f, "\n") 
   } else {
    &p1
   }
}
}
