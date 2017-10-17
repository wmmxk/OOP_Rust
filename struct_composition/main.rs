//source: https://www.reddit.com/r/rust/comments/3gc6dv/oop_in_rust/ 
// This code is show how to use composition to do what inheritance does in c++

#[derive(Debug)]

struct Animal {
       name: String
}

impl Animal {
		fn say_name(&self)  {
            println!("{:?}",self.name);
		}
}

struct Cat {

    animal: Animal
}


impl Cat {
		fn say_name(&self) {
           self.animal.say_name();
		}
}



fn main() {
		let v = Cat {
            animal:Animal{name:format!("Bob")
            }
		};
        v.say_name();
}
