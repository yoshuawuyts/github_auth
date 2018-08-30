use auth_response::AuthResponse;
use dialoguer::{Input, PasswordInput};
use directories::ProjectDirs;
use failure::Error;
use mkdirp::mkdirp;
use reqwest::{
  header::{ContentType, Headers, UserAgent},
  Client,
};
use serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::Builder;
use super::Scope;
use super::Token;

/// A GitHub auth instance.
#[derive(Debug)]
pub struct Authenticator {
  pub(crate) config: Builder,
}

/// An authentication returned by a GitHub auth instance.
#[derive(Debug)]
pub struct Authentication {
  /// The User's username.
  username: String,
  /// The token for the User.
  token: String,
}

/// An authentication request for GitHub
#[derive(Debug, Serialize)]
struct AuthRequest {
  note: String,
  scopes: Option<Vec<Scope>>,
}

impl AuthRequest {
  /// Create a new instance.
  pub fn new(note: String, scopes: Option<Vec<Scope>>) -> Self {
    Self { note, scopes }
  }
}

impl Authenticator {
  /// Create a new instance with no scopes allowed.
  pub fn new(name: String) -> Self {
    Builder::new(name).build()
  }

  /// Create a new instance and configure it.
  pub fn builder(name: String) -> Builder {
    Builder::new(name)
  }

  /// Get the location at which the token is stored.
  pub fn location(&self) -> PathBuf {
    let dirs = ProjectDirs::from("com", "GitHub Auth", &self.config.name);
    let dir = dirs.data_dir();
    let filename = dir.join("token.json");
    filename
  }

  /// Authenticate with GitHub.
  pub fn auth(&self) -> Result<Token, Error> {
    let dirs = ProjectDirs::from("com", "GitHub Auth", &self.config.name);
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

    // Create HTTP headers
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("X-GitHub-OTP", otp);
    headers.set(UserAgent::new("github_auth"));
    headers.set(ContentType::json());

    // Create HTTP body
    let note = self.config.note.clone();
    let scopes = self.config.scopes.clone();
    let body = AuthRequest::new(note, scopes);

    // Perform HTTP request.
    let mut res = client
      .post(::GITHUB_AUTH_URL)
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
    Builder::new("GitHub Auth".into())
      .note("A token created with the github_auth Rust crate.".into())
      .build()
  }
}
