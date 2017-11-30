#[derive(Debug)]
struct Node<'a> {
next: Option<&'a Node<'a>>,
data: i32,
}



impl<'a> Node<'a> {
	fn new_link(key:i32) -> Option<&'a Node<'a>> {

  //The newly created node is a local variable, we can not return it
  //If it is Box, which is owning pointer and the data is created on the heap, it is OK.
     Some(& Node{next:None,data:key })
	}
  
	fn append(&mut self, key: i32) {
     self.next = Node::new_link(key);
	}
}


fn main(){
let n1 = Node{next: None,data:3};
println!("{:?}", n1);

}
