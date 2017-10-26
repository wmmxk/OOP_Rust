#[derive(PartialEq,Debug)] 
enum Ordering {
    Less(usize),
    Equal,
    Greater(usize),

}

fn cmp(a: usize, b: usize) -> Ordering {
    if a < b { Ordering::Less(a-b)  }
    else if a > b { Ordering::Greater(a-b)  }
    else { Ordering::Equal  }

}

fn main() {
    let x = 5usize;
    let y = 10usize;

    let ordering = cmp(x, y);
    println!("ordering: {:?}", ordering);

	if ordering == Ordering::Less(ref local) {
        println!("less");
    
	} else if ordering == Ordering::Greater(_) {
        println!("greater");
    
	} else if ordering == Ordering::Equal {
        println!("equal");
    
	}

}
