//source:http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html

//A string is mmutable. As such, a String maintains a length and a capacity whereas as str only has a len() method. 

//&String is a reference to a String, also called a borrowed type. This is nothing more than a pointer which you can pass around without giving up ownership. Turns out a &String can be coerced to a &str;

//  As such, you almost never need to deal with &Strings. The only real use case I can think of is if you want to pass a mutable reference to a function that needs to modify the string:

fn main() {
    let mut s = String::from("Hello, Rust!");
    foo(&s);
    append(&mut s);    

}

fn foo(s: &str) {
    println!("{}", s);

}

fn append(s: &mut String) {
  s.push_str("Appending ");
  println!("{}",s);

}
