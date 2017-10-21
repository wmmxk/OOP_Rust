//source https://stackoverflow.com/questions/15619320/how-to-access-command-line-parameters

use std::env;

fn main() {
		if let Some(arg1) = env::args().nth(1) {
        println!("The first argument is {}", arg1);
    
		}

}
