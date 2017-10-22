#[macro_use]
extern crate simple_error;

use std::error::Error;

type BoxResult<T> = Result<T,Box<Error>>;

fn run(s: &str) -> BoxResult<i32> {
		if s.len() == 0 {
        bail!("empty string");
    
		}
    Ok(s.trim().parse()?)

}

fn main() {
    println!("{:?}", run("23"));
    println!("{:?}", run("2x"));
    println!("{:?}", run(""));

}
// Ok(23)
// Err(ParseIntError { kind: InvalidDigit  })
// Err(StringError("empty string"))

