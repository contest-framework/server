use schemars::JsonSchema;
use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum VarSource {
  File,
  Line,
  CurrentOrAboveLineContent,
}

impl Display for VarSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = match &self {
      VarSource::File => "file",
      VarSource::Line => "line",
      VarSource::CurrentOrAboveLineContent => "currentOrAboveLineContent",
    };
    write!(f, "{text}")
  }
}
