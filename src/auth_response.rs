#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthResponse {
  pub token: String,
}
