use std::fs;

fn main() {
  println!("JSON Schema export");
  let data = "Some data!";
  fs::write("schema.json", data).unwrap();
}
