use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
   let path  = Path::new("data.txt");
   let mut file = match File::open(&path) {
     Ok(file) => file,
	 Err(err) => {
              println!("Error while open file {}",err);
              panic!();
			 },
   };
   let mut s = String::new();
   file.read_to_string(&mut s);

   println!("Read the string: {}",s);



}
