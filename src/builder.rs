use super::Scope;

/// Create a new [`Authenticator`] instance.
#[derive(Debug, Default)]
pub struct Builder {
  pub(crate) scopes: Option<Vec<Scope>>,
  pub(crate) name: String,
  pub(crate) note: String,
}

impl Builder {
  /// Create a new instance.
  pub fn new(name: String) -> Self {
    let note = format!("A token created for {}.", &name);

    Self {
      name,
      note,
      ..Default::default()
    }
  }

  /// Set a custom note for the token stored on GitHub. Defaults to mentioning
  /// the token name.
  pub fn note(mut self, note: String) -> Self {
    self.note = note;
    self
  }

  /// Add a scope. [Read more.](https://developer.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/)
  pub fn scope(mut self, scope: Scope) -> Self {
    if let None = self.scopes {
      self.scopes = Some(vec![]);
    }

    if let Some(ref mut scopes) = &mut self.scopes {
      scopes.push(scope);
    }

    self
  }

  /// Finalize the builder, and return an `Authenticator` instance.
  pub fn build(self) -> ::Authenticator {
    ::Authenticator { config: self }
  }
}
