use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

fn test_write() {

  let tmp_dir = TempDir::new("example")?;
  let file_path = tmp_dir.path().join("my-temporary-note.txt");
  let mut tmp_file = File::create(file_path)?;
  writeln!(tmp_file, "Brain was here")?;



}

fn main() {
  test_write();

}
