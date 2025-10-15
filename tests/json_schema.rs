use contest::config::file::FileConfiguration;
use schemars::schema_for;
use std::fs;

#[test]
fn export_json_schema() {
  let schema = schema_for!(FileConfiguration);
  let text = serde_json::to_string_pretty(&schema).unwrap() + "\n";
  fs::write("documentation/schema.json", text).unwrap();
}
