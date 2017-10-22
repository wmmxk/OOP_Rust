
//source:https://stackoverflow.com/questions/25182565/how-to-pass-anonymous-functions-as-parameters-in-rust
//We define a generic type constrained to one of the closure traits: FnOnce, FnMut, or Fn.

fn main() {
    thing_to_do(able_to_pass);

	thing_to_do(|| {
        println!("works!");
    
					});

}

fn thing_to_do<F: FnOnce()>(func: F) {
    func();

}

fn able_to_pass() {
    println!("works!");

}
