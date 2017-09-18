#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
enum Fruit {
    Orange,
    // The `bool` value represents if the banana is ripe.
    Banana(bool),
    Apple{ color: Color },
}

fn print(x: Fruit) {
    match x {
        // The name `color` is a reference to `color` in `Fruit::Apple(color)`
        // so no move of values here.
        Fruit::Apple{ ref color } => 
            // No need to dereference `color` because of [auto-deref](todo.md)
            println!("Apple is {:?}", color),
        _ => (),
    }

    // OK
    println!("{:?}", x);
}

fn main() {
	let  mycolor  = Color::Red;
    let myfruit = Fruit::Apple{ color: mycolor};
    print(myfruit);
    println!("color enum I defin {:?}", mycolor);
}
