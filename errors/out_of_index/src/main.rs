//source: https://doc.rust-lang.org/book/second-edition/ch09-01-unrecoverable-errors-with-panic.html


// how to backtrace the error:
// RUST_BACKTRACE=1 cargo run  . it wil trace to the line in the main function

fn main() {
    let  arr=vec![1,2];
    for i in 0..5 {
    println!("{}",arr[i]);

}
}
