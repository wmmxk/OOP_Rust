// source: http://arthurtw.github.io/2014/11/30/rust-borrow-lifetimes.html

// Druing a mutable borrow, the owner can not lend the data 
// In the example x is borrowing the data, if it is not put in a {}, there will be an error

struct Foo {
    f: Box<i8>,

}


fn main() {
    let mut a = Foo { f: Box::new(0)  };
	{
        // mutable borrow
        let x = &mut a;
        // mutable borrow ends here
    
	}
    println!("the value of f {}", a.f);

}
