extern crate github_auth;

use github_auth::{Authenticator, Scope};

fn main() {
    let auth = Authenticator::builder("github_auth main example".into())
        .scope(Scope::PublicRepo)
        .build();

    let token = auth.auth().unwrap();
    println!("{:?}", token);

    let location = auth.location();
    println!("Token stored at: {:?}", location);
}
