extern crate github_auth;

use github_auth::Authenticator;

fn main() {
  let auth = Authenticator::default();
  let creds = auth.auth().unwrap();
  println!("{:?}", creds);
}
