//source: https://stackoverflow.com/questions/31406840/lifetime-bound-on-generic-parameter-not-required-on-impl-block

#[derive(Debug)]
struct SliceWrapper<'a, T: 'a> {
    a: &'a [T],
}

impl<'a, T> SliceWrapper<'a, T> {
    fn new(n: &'a [T]) -> SliceWrapper<'a, T> {
        SliceWrapper { a: n }
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];

    let aw = SliceWrapper::new(&array[..2]);

    println!("{:?}", aw);
}
