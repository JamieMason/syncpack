use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct WorkspaceProtocol {
  pub raw: String,
}

impl WorkspaceProtocol {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::WorkspaceProtocol(Self { raw: raw.to_string() })
  }
}
