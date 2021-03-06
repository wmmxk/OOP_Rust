//This is to show:
// 1. it is not allowed to move data from a borrowed reference
// 2. In a match pattern, you can not use & and ref interchangably
// 3. double reference, if I use let ref d2 = &d1; It means d2 is a reference to the address of d1 not a reference to d1;
#[derive(Debug)]
enum Direction {
     East(String),
     West(isize),
}
fn main()
{

  let d1 = Direction::East(String::from("east"));

  let ref d2;
  d2 = &d1;
  match d2 {
     // you need use ref not & because local does not exist before you destructure the 
     &Direction::East(ref local) => println!("num {} ", local),
     &Direction::West(local) => println!("Direction {:?} ", local),
  }
}
