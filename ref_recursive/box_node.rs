#[derive(Debug)]
struct Node {
next: Option<i32>,
data: i32,
}



impl Node {
	fn new_link(key:i32) -> Option<Box<Node>> {

     Some(Box::new(Node{next:None,data:key }))
	}
  
}


fn main(){
let key = 3;
println!("{:?}", Node::new_link(key));

}
