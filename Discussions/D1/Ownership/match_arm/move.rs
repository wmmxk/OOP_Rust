// The example is derived from this example : https://doc.rust-lang.org/1.6.0/book/patterns.html  (search ref you will see the example)
// Explaination: 

     //1. pattern matching:
         // In the scope of a match arm, the data is matched to local variable
       

     // 2. move vs borrow. 
     // Types like integers have a known size at compile time and are stored entirely on the stack,
     // so copies of the actual values are quick to make. 
     // https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html

     // 3. traits. 
     // This example also shows the same operation behaviors different on different types of types. 
     // The trait of a type specifies the behavior of an operation on a type. In this case, the String type
     // does not have the Copy trait, so the older variable is not usable after assignment.


fn main() {
 
    let y = 5;
	match y {
         localv => println!("local v is {} ",localv),
	}

    let z = y;
    println!("z is {}",z);


    let s1 = String::from("hello");
    println!("s1 is {}", s1);
	match s1 {

// after the ownership is given to the localv, you can not use s1 anymore immediately.
         localv => println!("local v is {} ",s1),
//correct          ref localv => println!("local v is {} ",localv),
	}


  
}
