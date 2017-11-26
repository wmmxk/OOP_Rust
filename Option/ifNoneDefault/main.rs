//source: https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr

// unwrap_or: unwrap an data of Option type; if it is None return a default value

fn main() {
   let opt1: Option<String> = Some("some value".to_owned());
   let v1 = opt1.as_ref().map(|x| &**x).unwrap_or("default string");

   let opt2 : Option<String> = None;
   let v2 =  opt2.as_ref().map(|x| &**x).unwrap_or("default string");

   println!("{:?}", v1);
   println!("{:?}",v2);


}
