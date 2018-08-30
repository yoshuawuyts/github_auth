#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate failure;
extern crate dialoguer;
extern crate directories;
extern crate mkdirp;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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
pub const GITHUB_AUTH_URL: &'static str =
  "https://api.github.com/authorizations";
