extern crate github_auth;

use github_auth::{Authenticator, Scope};

#[async_std::main]
async fn main() -> surf::Result<()> {
    let auth = Authenticator::builder("github_auth main example".into())
        .scope(Scope::PublicRepo)
        .build();

    let token = auth.auth().await?;
    println!("{:?}", token);

    let location = auth.location();
    println!("Token stored at: {:?}", location);
    Ok(())
}
