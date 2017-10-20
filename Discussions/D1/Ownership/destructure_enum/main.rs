// source: https://github.com/rust-lang/rust/issues/40666
// the first time you destructure a variant, use a ref variable to bind the data, otherwise, the data is consumed
#[derive(Debug)]
enum myenum {
   Number,
   Character {x : String, y: String},
}


fn main() {
   let aenum = myenum::Character{x:String::from("hello"),y: String::from("world")};


   match aenum {
		    myenum::Character { ref x, ref y} => {
             println!("The value of x: {} ", *x )
		   },
           _ => {println!("Others ")},
   }


   match aenum {
		    myenum::Character {x, y} => {
             println!("The value of x: {} ",x )
		   },
           _ => {println!("Others ")},
   }


}
