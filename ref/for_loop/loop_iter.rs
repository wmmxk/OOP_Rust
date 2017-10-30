//source: https://doc.rust-lang.org/book/second-edition/ch10-01-syntax.html
// How does the move happen in &str in str.iter()
//

fn main() {
let strs = vec![String::from("s1"),String::from("s2")];
for  &str in strs.iter() {

    println!("each string {:?}:", str);
}

//   println!("1st element {:?}", strs[0]);


}
