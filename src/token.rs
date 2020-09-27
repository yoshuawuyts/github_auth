use std::fmt;

/// Token returned by the authenticator function.
#[derive(Debug, PartialEq)]
pub struct Token(String);

impl Token {
    pub(crate) fn new(bytes: String) -> Self {
        Token(bytes)
    }

    /// Convert to a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
