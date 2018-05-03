/// Token returned by the authenticator function.
#[derive(Debug, PartialEq)]
pub struct Token(Vec<u8>);

impl Token {
  pub(crate) fn new(bytes: Vec<u8>) -> Self {
    Token(bytes)
  }
}
