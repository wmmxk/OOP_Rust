// This is to get a taste of methods offered by an iterator
//source: https://killercup.github.io/trpl-ebook/trpl-2016-10-01.html#sec--iterators
// the ::<> syntax allows us to give a type hint, Otherwise Rust can't determine what type of things you want to collect

fn main() {
    let range = (0..10).collect::<Vec<i32>>();
    println!("{:?}", &range[..5]);
    println!("{:?}", &range[2..5]);
    println!("{:?}", &range[7..]);
}
