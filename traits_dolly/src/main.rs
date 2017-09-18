//structure of this code: 
//1. declare and implement a struct data type
//2. define a trait
//3. implement the trait for your data type. (the trait add more methods
//   to your data type. e.g the name() method)
//4. 


struct Sheep {naked: bool, name: &'static str}
impl Sheep {
	fn is_naked(&self) -> bool {
		self.naked
	}
		
	fn shear (&mut self) {
		if self.is_naked() {
			println!("{} is a already naked...",self.name);
		} else {
			println!("{} gets a haircut!", self.name);
			self.naked = true;
		}
	}
}

trait Animal {
	fn new(name: &'static str) -> Self;
	fn name(&self) -> &'static str;
	fn noise(&self) -> &'static str;

	fn talk(&self) {
		println!("{} says {}", self.name(), self.noise());
	}
}

impl Animal for Sheep {
	fn new(name: &'static str) -> Sheep {
		Sheep { name: name, naked: false }
	}
	fn name(&self) -> &'static str {
		self.name
	}
	fn noise(&self) -> &'static str {
		if self.is_naked() {
			"baaah?"
		} else {
			"baaah!"
		}
	}
	fn talk(&self) {
		println!("{} pauses briefly...{}", self.name, self.noise());
	}

}


fn main() {
	let mut dolly: Sheep = Animal::new("Dolly");
	
	dolly.talk();
	dolly.shear();
	dolly.talk();
}
