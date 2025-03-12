use std::fs;

#[test]
fn export_json_schema() {
  let data = "Some data!";
  fs::write("schema.json", data).unwrap();
}
