mod auth_response;
mod authenticator;
mod builder;
mod scopes;
mod token;

pub use authenticator::*;
pub use builder::*;
pub use scopes::*;
pub use token::*;

/// The GitHub authorization base URL
pub const GITHUB_AUTH_URL: &'static str = "https://api.github.com/authorizations";
