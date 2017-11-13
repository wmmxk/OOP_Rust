type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    key: T,
    next: Link<T>
}

impl<T> Node<T> {
   fn new_link(key:T) -> Link<T>{
   Some(Box::new(Node{key:key,next:None}))
}
}

struct Llist<T> {
  head: Link<T>,
}


impl<T: std::fmt::Debug> Llist<T> {
   fn print_values(&mut self)  {
      let mut next = &mut self.head;
      loop {
         let next_ = next;
         match *next_ {
            Some(ref mut node) => {println!("{:?} ", node.key);
                                  next = &mut node.next;
                                  },
            None => {break;},
         }
      }
}
}

fn main() {
let ref mut  l1 = Llist {head:None};
(*l1).head = Node::new_link(4);
(l1).print_values();

}
