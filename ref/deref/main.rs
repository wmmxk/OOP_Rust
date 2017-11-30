//source: https://doc.rust-lang.org/book/second-edition/ch15-02-deref.html

//In the textbook, it is said there should be a compilation error, but no error
fn main() {
   let x = 5;
   let y = &x;
   assert_eq!(5,x);
   assert_eq!(5,*y);

}
