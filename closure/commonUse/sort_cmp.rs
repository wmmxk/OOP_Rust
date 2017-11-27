//source1: https://doc.rust-lang.org/core/cmp/enum.Ordering.html
//source2: http://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
//use std::cmp;
//
fn main() {
let mut data = & mut [2,10,5];
data.sort_by(|a,b| a.cmp(b).reverse());
println!("{:?}", &data);

let mut teams = [("Jack", 80), ("Jane", 98),("John",78)];
teams.sort_by(|&a, &b| a.1.cmp(&b.1));
println!("{:?}", &teams);
}
