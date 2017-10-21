// source: http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// Let’s think about this. We know we want to hint to the Rust compiler that our struct Person should not outlive name because the struct does not own the str
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
		fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    
		}

}

fn main() {
    let person = Person { name: "Herman"  };
    person.greet();

}
