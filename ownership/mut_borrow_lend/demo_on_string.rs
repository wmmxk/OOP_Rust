// source: http://arthurtw.github.io/2014/11/30/rust-borrow-lifetimes.html

// Druing a mutable borrow, the owner can not lend the data 
// In the example x is borrowing the data, if it is not put in a {}, there will be an error



fn main() {
    let mut  s1 = String::from("str1");

        // mutable borrow
        let s2 = &mut s1;
        // mutable borrow ends here
    
    println!("the value of f {:?}", s1);

}
