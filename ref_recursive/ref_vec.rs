#[derive(Debug)]
struct Node<'a> {
tag: &'a mut Vec<i32>,
}

impl<'a> Node<'a> {
		pub fn insert(&mut self, t: i32) {
           self.tag.push(t);
		}

}


fn main(){
let mut v1 = vec![1,2,3];
let mut n1 = Node{tag: & mut v1};
let t =5;
n1.insert(t);

println!("{:?}", n1);


}
