//source: https://doc.rust-lang.org/1.6.0/book/patterns.html
// This is to show how local binding in the scope of match arm works

fn main() {


let x = 'x';
let c = 'c';

match c {
    x => println!("x: {} c: {}", x, c),
}

println!("x: {}", x)

}
