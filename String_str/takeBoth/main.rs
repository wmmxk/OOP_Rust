//source http://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
// when you search around as_ref, you stumbled upon this

//This is to create a function can take a string or &str
struct Person {
    name: String,
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

fn main() {
    let person = Person::new("Herman");
    let person = Person::new("Herman".to_string());
}
