//source: https://stackoverflow.com/questions/24831828/how-do-i-pass-an-array-to-a-function-in-rust-and-change-its-content


fn change_value(arr: &mut [i32]) {
    arr[1] = 10;
}

fn main() {
    let mut arr: [i32;4] = [1, 2, 3, 4];
    change_value(&mut arr);
    change_value(&mut arr[1 .. 3]);
    println!("this is {}", arr[1]);

    println!("this is {}", arr[2]);
        
}
