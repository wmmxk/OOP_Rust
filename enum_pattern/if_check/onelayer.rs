#[derive(PartialEq,Debug)] 
enum Ordering {
    Less,
    Equal,
    Greater,

}

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less  }
    else if a > b { Ordering::Greater  }
    else { Ordering::Equal  }

}

fn main() {
    let x = 5u32;
    let y = 10u32;

    let ordering = cmp(x, y);
    println!("ordering: {:?}", ordering);
	if ordering == Ordering::Less {
        println!("less");
    
	} else if ordering == Ordering::Greater {
        println!("greater");
    
	} else if ordering == Ordering::Equal {
        println!("equal");
    
	}

}
