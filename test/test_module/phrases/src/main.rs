//source https://doc.rust-lang.org/book/first-edition/crates-and-modules.html
extern crate phrases;

fn main() {
   println!("Hello in English: {}", phrases::english::greetings::hello());

   println!("Hello in English: {}", phrases::japanese::greetings::hello());

}
