// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#[derive(Debug)]

// Create an `enum` to classify someone. Note how both names
// and type information together specify the variant:
// `Engineer != Scientist` and `Height(i32) != Weight(i32)`. Each
// is different and independent.
enum Origin {
	Europe,
	China,
}
enum Person {
    // An `enum` may either be `unit-like`,
    Scientist,
	Nation_info {nation: Origin},
    // like tuple structs,
    Height(i32),
    // or like structures.
    Info { name: String, height: i32 }
}

// A function which takes a `Person` enum as an argument and
// returns nothing.
fn inspect(p: Person) {
    // Usage of an `enum` must cover all cases (irrefutable)
    // so a `match` is used to branch over it.
    match p {
        Person::Scientist => println!("Is a scientist!"),
        // Destructure `i` from inside the `enum`.
        Person::Height(i) => println!("Has a height of {}.", i),
        // Destructure `Info` into `name` and `height`.
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
		Person::Nation_info {nation} => {println!("you are from China");},
    }
}

fn main() {

    let rebecca  = Person::Scientist;
    let person   = Person::Height(18);
    // `to_owned()` creates an owned `String` from a string slice.
    let dave     = Person::Info { name: "Dave".to_owned(), height: 72 };
    let myorigin = Origin::China;
    println!("{:?}", & myorigin);
    let me = Person::Nation_info {nation:  myorigin};
    inspect(rebecca);
	inspect(person);
    inspect(dave);
    inspect(me);
}
