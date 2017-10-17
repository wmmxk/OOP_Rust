//source: http://events.linuxfoundation.org/sites/events/files/slides/rust-4-cpp2.pdf

// reason for the error:
//  You can define two mutable borrow in one scope

fn main() {

    let mut v = vec![];
    v.push("Hello");
 
    let x = &mut v[0];

    v.push("word");

    println!("{}",x);

}

