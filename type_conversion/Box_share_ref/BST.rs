#[derive(Debug)]
struct Tree<T>  {
data: Option<T>,
left: Option<Box<Tree<T>>>,
right: Option<Box<Tree<T>>>,
}
impl<T: Ord> Tree<T> {
	fn insert(&mut self, key:T) -> bool {
		match self.data {
			None => {
         self.data = Some(key);
         return true
			}
			Some(ref value) => {
				if *value == key {
           return false
				}
				else if *value >  key {
					match self.left {
             Some(ref mut node) =>
						 {
              node.as_mut().insert(key)
						 },
             None => 
             {
              self.left = Some(Box::new(Tree{data:Some(key),left:None, right:None}));
             true
             }  
					}
				}
				else {
					match self.right {
             Some(ref mut node) =>
						 {
               node.as_mut().insert(key)
						 },
             None => 
             {
              self.right = Some(Box::new(Tree{data:Some(key),left:None, right:None}));
             true
             }  
          }
			}
		}
	}
}
}
fn main(){ 
let mut t1: Tree<i32> = Tree{data:Some(3), left: None, right:None};
t1.insert(5);
println!("{:?}", t1); 
 
 } 
