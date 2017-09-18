//structure of this code: 
//1. declare and implement a struct data type
//2. define a trait
//3. implement the trait for your data type. (the trait add more methods
//   to your data type. e.g the name() method)
//4. 

struct Rectangle {

	x: i32,
	y: i32,
	w: i32,
	h: i32,
}

struct Square {
	x: i32,
	y: i32,
	w: i32,
}

trait Area {

	fn area(&self) -> i32;
	fn new() -> Self;
}


impl Area for Rectangle {

	fn area(&self) -> i32 {
		self.w * self.h
	}

	fn new() -> Self {
		Rectangle{x:0, y:0, w:2, h:1}
	}
}

impl Area for Square {
	fn area(&self) -> i32 {
		self.w * self.w
	}
	fn new() -> Self {
		Square{x:0, y:0, w:1}
	}

}

fn print_area<T: Area>(x: T) {

	println!("The area is {}", x.area());
}

fn new_area<T: Area> () -> T {
	T::new()
}


fn main() {
	let x: Rectangle = new_area();
	let y = new_area::<Square>();

	print_area(x);
	print_area(y);

}
