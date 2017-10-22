//source https://doc.rust-lang.org/book/first-edition/error-handling.html
//argv.nth(1) return an Option; arg.parse() produces a Result. So if argv.nth(1) is not Ok, that is a None is returned, return a Rusult with an error

//2. if Ok, then do the parse. 

use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)

}

fn main() {
		match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    
		}

}
