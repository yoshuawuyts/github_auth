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
mod token;

use self::auth_response::AuthResponse;
use self::dialoguer::{Input, PasswordInput};
use self::directories::ProjectDirs;
use self::failure::Error;
use self::mkdirp::mkdirp;
use self::reqwest::{
  header::{ContentType, Headers, UserAgent},
  Client,
};
use self::token::Token;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const GITHUB_URL: &'static str = "https://api.github.com/authorizations";

/// Configuration passed to create a new GitHub auth instance.
#[derive(Debug, Default)]
pub struct Config {
  /// GitHub auth scopes. E.g. `['user']`.
  // FIXME: convert scopes into a Vector of an Enum. Implement serializability
  // on it too.
  pub scopes: Option<Vec<String>>,

  /// Saved with the token on GitHub. Allows you to identify the purpose of this
  /// token from the GitHub UI.
  pub note: String,

  /// User agent used to make a request.
  pub user_agent: Option<String>,
}

/// A GitHub auth instance.
#[derive(Debug)]
pub struct Authenticator {
  name: String,
  config: Config,
}

/// An authentication returned by a GitHub auth instance.
#[derive(Debug)]
pub struct Authentication {
  /// The User's username.
  username: String,
  /// The token for the User.
  token: String,
}

impl Authenticator {
  /// Create a new instance.
  pub fn new(name: String, config: Config) -> Self {
    Authenticator { name, config }
  }

  /// Get the location at which the token is stored.
  pub fn location(&self) -> PathBuf {
    let dirs = ProjectDirs::from("com", "GitHub Auth", &self.name);
    let dir = dirs.data_dir();
    let filename = dir.join("token.json");
    filename
  }

  /// Authenticate with GitHub.
  pub fn auth(&self) -> Result<Token, Error> {
    let dirs = ProjectDirs::from("com", "GitHub Auth", &self.name);
    let dir = dirs.data_dir();
    mkdirp(&dir)?;
    let filename = dir.join("token.json");

    if let Ok(mut file) = File::open(&filename) {
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      let json: AuthResponse = serde_json::from_str(&contents)?;
      return Ok(Token::new(json.token));
    }

    // Get CLI input.
    let username = Input::new("GitHub username").interact()?;
    let password = PasswordInput::new("GitHub password").interact()?;
    let otp = Input::new("GitHub OTP (optional)").interact()?;

    // Perform HTTP request.
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("X-GitHub-OTP", otp);
    headers.set(UserAgent::new("github_auth"));
    headers.set(ContentType::json());
    let mut body = HashMap::new();
    // if let Some(scopes) = self.config.scopes {
    //   body.insert("scopes", *scopes);
    // }
    body.insert("note", &self.config.note);
    let mut res = client
      .post(GITHUB_URL)
      .json(&body)
      .headers(headers)
      .basic_auth(username, Some(password))
      .send()?;

    // Parse request output.
    let status = res.status();
    ensure!(
      status.is_success(),
      format!(
        "{:?} {:?}",
        res.text().unwrap(),
        status.canonical_reason().unwrap()
      )
    );

    let json: AuthResponse = res.json()?;

    let serialized = serde_json::to_string(&json)?;
    let mut file = File::create(&filename)?;
    file.write_all(&serialized.as_bytes())?;

    Ok(Token::new(json.token))
  }
}

impl Default for Authenticator {
  /// Create a new instance of
  fn default() -> Self {
    let mut config = Config::default();
    config.note = String::from(
      "An unidentified token created with the github_auth Rust crate.",
    );
    Authenticator {
      name: String::from("GitHub Auth"),
      config,
    }
  }
}
