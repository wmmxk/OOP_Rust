#![allow(unused_variables)]

fn main() {
#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Green,
}

enum Fruit {
    Orange,
    Banana(bool),

    Apple{color: Color},
}
fn print(x: Fruit) {
    match x {
        Fruit::Orange => println!("Orange"),
        Fruit::Banana(ripe) => println!("Banana is {}",
            if ripe {"ripe"} else {"raw"}),
        Fruit::Apple{ref color } => println!("Apple is {:?}",color)
    }
}
    let someColor = Color::Red;
    let someApple = Fruit::Apple{color:someColor};
  
    print(someApple);
}
