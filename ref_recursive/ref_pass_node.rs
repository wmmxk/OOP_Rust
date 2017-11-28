#[derive(Debug)]
struct Node {
next: Option<i32>,
data: i32,
}


impl <'a> Node {
	fn new_link(key:i32) -> Option<&'a Node> {

     Some(&(Node{next:None,data:key }))
	}
  
}


fn main(){
let key = 3;
println!("{:?}", Node::new_link(key));

}
