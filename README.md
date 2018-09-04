# github_auth
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Authenticate with GitHub from the command line. Caches the authentication token
so that future interactions just work.

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust,ignore
extern crate github_auth;

use github_auth::{Authenticator, Scope};

let auth = Authenticator::builder("github_auth main example".into())
  .scope(Scope::PublicRepo)
  .build();

let token = auth.auth().unwrap();
println!("{:?}", token);

let location = auth.location();
println!("Token stored at: {:?}", location);
```

## Example Output
This dialog is only required to generate a valid token. Once a valid token is
created, it will no longer be shown.
```txt
GitHub username: my_name
GitHub password:
GitHub OTP (optional): 5678
```

## Authenticating with the token
Once you've acquired an access token, [you can use it to
authenticate](https://developer.github.com/apps/building-oauth-apps/authorization-options-for-oauth-apps/#3-use-the-access-token-to-access-the-api).
Here's how to authenticate with the [reqwest](http://docs.rs/reqwest) crate.
```rust,ignore
extern crate github_auth;
extern crate reqwest;

use github_auth::Authenticator;
use reqwest::{
  header::{Authorization, Headers, UserAgent},
  Client,
};

let auth = Authenticator::new("my_example_app");
let token = auth.auth().unwrap();

let mut headers = Headers::new();
headers.set(Authorization(format!("token {}", token.as_str())).to_owned());
headers.set(UserAgent::new("my_app"));

let url = "https://api.github.com/user";
let mut res = client.get(&url).headers(headers).send()?;
println!("{:?}", res.status());
```

## Installation
```sh
$ cargo add github_auth
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/github_auth.svg?style=flat-square
[2]: https://crates.io/crates/github_auth
[3]: https://img.shields.io/travis/yoshuawuyts/github_auth.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/github_auth
[5]: https://img.shields.io/crates/d/github_auth.svg?style=flat-square
[6]: https://crates.io/crates/github_auth
[7]: https://docs.rs/github_auth/badge.svg
[8]: https://docs.rs/github_auth
