//step_by does not work
// any; find;
fn main() {
	for i in (0..6).take(3) {
    println!("{}",i);
  }

	for i in (0..6).skip(2) {
    println!("{}",i);
  
  }

	for i in (0..3).cycle().take(6) {
    println!("{}",i);
  }

  println!("Filter");
	for i in (0..5).filter(|&item| item%2 ==0) {
    println!("{}",i);
  }

  println!("reverse");
	for i in (0..3).rev(){
    println!("{}",i);
  }

	println!("{:?}", (0..3).find(|&x| x ==1));


     let array = [1u32, 3, 3, 7];

// When you need the behavior of both of queue and a stack, you can use the next() and next_back().
    println!("flanked {:?}", array.iter().next_back());

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
 
  

    let arr1 = [0,1,2];
    let arr2 = [3,4];
    let ite1 = arr1.iter();
    let ite2 = arr2.iter();

		for (i,j) in ite1.zip(ite2) {
      println!("{}, {}",i,j);
		}
    

}
