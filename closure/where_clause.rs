
//source:https://stackoverflow.com/questions/25182565/how-to-pass-anonymous-functions-as-parameters-in-rust
//you can use a where clause to constrain the generic type

fn main() {
    thing_to_do(able_to_pass);

	thing_to_do(|| {
        println!("works!");
    
					});

}

fn thing_to_do<F>(func: F) 
    where F: FnOnce(),
{
		  {
            func();
		  }
}

fn able_to_pass() {
    println!("works!");

}
