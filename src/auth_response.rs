#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthResponse {
  pub token: Vec<u8>,
}
