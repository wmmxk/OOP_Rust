//source: https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html
fn num_to_ordinal(x: u32) -> String {
    let suffix;
    match (x % 10, x % 100) {
        (1, 1) | (1, 21...91) => {
            suffix = "st";
        }
        (2, 2) | (2, 22...92) => {
            suffix = "nd";
        }
        (3, 3) | (3, 23...93) => {
            suffix = "rd";
        }
        _                     => {
            suffix = "th";
        }
    }
    return format!("{}{}", x, suffix);
}

fn main() {
    assert_eq!(num_to_ordinal(   0),    "0th");
    println!(" res: {}",num_to_ordinal(   1));
    assert_eq!(num_to_ordinal(  12),   "1th");
    assert_eq!(num_to_ordinal(  22),   "22nd");
    assert_eq!(num_to_ordinal(  43),   "43rd");
    assert_eq!(num_to_ordinal(  67),   "67th");
    assert_eq!(num_to_ordinal(1901), "1901st");
}
