//This is to show:
// 1. it is not allowed to move data from a borrowed reference
// 2. In a match pattern, you can not use & and ref interchangably
// 3. double reference
#[derive(Debug)]
enum Direction {
     East(String),
     West(isize),
}
fn main()
{

  let d1 = Direction::East(String::from("east"));
// let ref d2 = &d1; the d2 will be a reference to the address of the d1 not d1, itself.
  let ref d2 = d1;
//  d2 = &d1;
  match d2 {
     &Direction::East(ref local) => println!("num {} ", local),
     &Direction::West(local) => println!("Direction {:?} ", local),
  }
}
