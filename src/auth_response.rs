use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthResponse {
    pub token: String,
}
