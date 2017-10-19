//source: https://stackoverflow.com/questions/29088633/grouping-structs-with-enums
// In the match arm, you don't need the field name; but when you create a variable of Command type, you need the field name


enum Command {
    Increment {quantity: u32},
    Decrement {quantity: u32},
}

impl Command {
    fn process(self) {
        match self {
            Command::Increment { quantity } => {
                println!("Increasing by: {}.", quantity)
            },
            Command::Decrement { quantity } => {
                println!("Decreasing by: {}.", quantity)
            },
        };
    }
}

fn main() {
    let input = "Add";
    let quantity = 4;

    let command: Command = match input {
        "Add" => Command::Increment { quantity: quantity },
        "Subtract" => Command::Decrement { quantity: quantity },
        _ => unreachable!(),
    };

    command.process();
}
