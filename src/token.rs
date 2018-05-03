/// Token returned by the authenticator function.
#[derive(Debug, PartialEq)]
pub struct Token(String);

impl Token {
  pub(crate) fn new(bytes: String) -> Self {
    Token(bytes)
  }
}
