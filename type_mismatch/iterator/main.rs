//source: https://doc.rust-lang.org/core/iter/trait.Iterator.html

//When invoking the next method on an iterator constructed from a vecotor, it returns a reference

struct CountUp {
	current: usize, 
}

impl Iterator for CountUp {
   type Item = usize;
	 fn next(&mut self) -> Option<usize> {
      self.current +=1;
      Some((self.current))
	 }
}
	

fn main() {
	let a = [1,2,3];
	let mut iter = a.iter();
	assert_eq!(Some(&1), iter.next());
   
  let mut counter = CountUp {current: 0};
  assert_eq!(Some(1), counter.next());
}
