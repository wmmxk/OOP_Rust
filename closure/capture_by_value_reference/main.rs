//source: https://rustbyexample.com/fn/closures/input_parameters.html
//The apply function takes a closure as an argument, if the closure captures by reference but the closure you passes captures by
// value. You will see an error

// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}


fn main() {
    use std::mem;

    let  farewell = String::from("Goodbye");

    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        let local = farewell;
        println!("{:?}", local); 
    };

    // Call the function which applies the closure.
    apply(diary);
//    println!("{:?}", farewell);
}
