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

use github_auth::Authenticator;

let auth = Authenticator::new("my_example_app");
let creds = auth.auth().unwrap();
println!("{:?}", creds);

let location = auth.location();
println!("Token is stored at {:?}", &location);
```

## Example Output
This dialog is only required to generate a valid token. Once a valid token is
created, it will no longer be shown.
```txt
GitHub username: my_name
GitHub password:
GitHub OTP (optional): 5678
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
