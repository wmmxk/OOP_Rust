#[derive(Debug)]
struct Node<'a> {
next: Option<&'a Node<'a>>,
data: i32,
}



fn main(){
let n1 = Node{next: None,data:3};
println!("{:?}", n1);

}
