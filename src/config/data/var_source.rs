use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Eq, PartialEq)]
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
