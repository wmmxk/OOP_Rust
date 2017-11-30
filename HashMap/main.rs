//source: https://hoverbear.org/2015/05/02/a-journey-into-iterators/
use std::collections::HashMap;
fn main() {
    // Initialize an input map.
    let mut input = HashMap::<u64, u64>::new();
    input.insert(1, 10); // Type inferred here.
    input.insert(2, 20);
    input.insert(3, 30);
    // Continue...
    let iterator = input.iter();
    let mapped = iterator.map(|(&key, &value)| {
            return (key, value * 10);
        });
    let output = mapped.collect::<Vec<_>>();
    println!("{:?}", output);
}
