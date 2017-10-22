// This code is to show if the data is borrowed by another variable, you can move the data if the borrow is still there
fn main(){
 
let x = String::from("Hello");
let y: &String = &x;
{
let z = x;
}
//println!("x: {}", x);
//println!("y: {}", y);



}
