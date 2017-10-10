// Each variant of an enum type has a unique and constant integral discriminator value. If no explicit discriminator is specified for a variant, the value defaults to the value of the previous variant plus one. If the first variant does not have a discriminator, it defaults to 0.  

#[derive(Debug)]
enum Direction {
     North,
     East,
     South=10,
     West,
}

fn main(){

println!("{:?} => {}", Direction::North, Direction::North as u16  );

println!("{:?} => {}", Direction::West, Direction::West as u16  );

println!("{:?} => {}", Direction::South, Direction::South as u16  );
}
