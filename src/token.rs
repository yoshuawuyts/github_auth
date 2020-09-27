/// Token returned by the authenticator function.
#[derive(Debug, PartialEq)]
pub struct Token(String);

impl Token {
    pub(crate) fn new(bytes: String) -> Self {
        Token(bytes)
    }

    /// Borrow as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to a string.
    pub fn into_string(self) -> String {
        self.0
    }
}
