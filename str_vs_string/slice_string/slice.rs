//source: https://doc.rust-lang.org/nightly/book/second-edition/ch04-03-slices.html
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
			if item == b' ' {
            return &s[0..i];
        
			}
    
	}

    &s[..]

}


fn main() {
   let mut s = String::from("hello buddy");
   let word = first_word(&s);
   println!("a slice: {}",word);

}
