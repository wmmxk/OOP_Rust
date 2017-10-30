// This code is to show you can not move out of indexed content

fn main() {
   let strs = vec![String::from("s1"),String::from("s2")];
   let ref_strs = & strs;
   let longest = & ref_strs[0];
   let l1 = strs[0];



}
