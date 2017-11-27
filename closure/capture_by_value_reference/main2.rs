//source:https://doc.rust-lang.org/std/ops/trait.FnMut.html

// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    F: FnOnce() {

    f();
}


fn main() {

    let  mut x = 5;
    {
    let mut square_x  = ||  x *=x;
    // Call the function which applies the closure.
    apply(square_x);
    }
    println!("{:?}", x);
}
