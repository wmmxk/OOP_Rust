//if you compile the main.rs, you will see an error pointed to line 2. No other hints. 

// How to fix: 
// The item_attribute should be put right before the struct defination

#[derive(Debug)]
use std::io::stdin;

struct Animal {
       name: String
}


fn main() {
 println!("Hello");
}
