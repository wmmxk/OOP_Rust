// The for loop does not return a Result type
// the std::ftm::Result takes no arguments

// type Result = Result<(), Error>;
fn myfun() -> std::fmt::Result {
   let x: i32 = 4;
   let y: i32 = 3;

   let j =3;
//   for i in 1..3 {
		   if j< 2 {
           Ok((1))
		   } else {
             Ok((5))
		   }
//   }
}

fn main() {

myfun();


}
