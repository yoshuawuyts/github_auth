use crate::auth_response::AuthResponse;
use dialoguer::{Input, PasswordInput};
use directories::ProjectDirs;
use serde::Serialize;
use surf::http::{bail, format_err, headers};
use surf::Body;

use std::fs::{self, remove_file, File};
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
    pub fn location(&self) -> surf::Result<PathBuf> {
        let dirs = ProjectDirs::from("com", "GitHub Auth", &self.config.name)
            .ok_or_else(|| format_err!("Could not access project dir for {}", &self.config.name))?;
        let dir = dirs.data_dir();
        let filename = dir.join("token.json");
        Ok(filename)
    }

    /// Remove the token from the local storage.
    pub fn delete(&self) -> surf::Result<()> {
        Ok(remove_file(self.location()?)?)
    }

    /// Authenticate with GitHub.
    pub async fn auth(&self) -> surf::Result<Token> {
        let dirs = ProjectDirs::from("com", "GitHub Auth", &self.config.name)
            .ok_or_else(|| format_err!("Could not access project dir for {}", &self.config.name))?;
        let dir = dirs.data_dir();
        fs::create_dir_all(&dir)?;
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

        // Create HTTP body
        let note = self.config.note.clone();
        let scopes = self.config.scopes.clone();
        let body = AuthRequest::new(note, scopes);

        // Encode username / password for basic auth.
        let mut auth_value = b"Basic ".to_vec();
        let mut encoder = base64::write::EncoderWriter::new(&mut auth_value, base64::STANDARD);
        write!(encoder, "{}:{}", username, password)?;
        drop(encoder);
        let auth_value = String::from_utf8(auth_value)?;

        // Perform HTTP request.
        let mut res = surf::post(crate::GITHUB_AUTH_URL)
            .header("X-GitHub-OTP", otp)
            .header("User-Agent", "github_auth")
            .header(headers::AUTHORIZATION, auth_value)
            .body(Body::from_json(&body)?)
            .await?;

        // Parse request output.
        let status = res.status();
        if !status.is_success() {
            bail!(
                "{:?} {:?}",
                res.body_string().await?,
                status.canonical_reason()
            );
        };

        let json: AuthResponse = res.body_json().await?;

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
