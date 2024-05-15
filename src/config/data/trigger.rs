#[derive(Debug, PartialEq)]
pub enum Trigger {
  TestAll,
  TestFile { files: glob::Pattern },
  TestFileLine { files: glob::Pattern },
}
