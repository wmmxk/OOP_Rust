// source: https://github.com/rust-lang/rust/issues/40666

#[derive(Debug)]
enum myenum {
   Number {x: i32, y:i32},
   Character,
}


fn main() {
   let aenum = myenum::Number{x:3,y:4};


   match aenum {
		    myenum::Number {x, y} => {
             println!("The value of x: {} ",x )
		   },
           _ => {println!("Character")},
   }

   match aenum {
		  avariant @  myenum::Number { ..  } => {
            println!("The variant: {:?} ", avariant )
		   },
           _ => {println!("Character")},
   }
}
