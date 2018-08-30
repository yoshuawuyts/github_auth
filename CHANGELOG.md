## 2018-08-30, Version 0.6.0
### Commits
- [[`3003bdce81`](https://github.com/yoshuawuyts/github_auth/commit/3003bdce8110f3a28437f3998249e2098fe969ea)] (cargo-release) version 0.6.0 (Yoshua Wuyts)
- [[`f2c164b407`](https://github.com/yoshuawuyts/github_auth/commit/f2c164b407980f21d13db62318e58cc21f04f55f)] Scopes (#2) (Yoshua Wuyts)
- [[`4b22873bab`](https://github.com/yoshuawuyts/github_auth/commit/4b22873babfca96afd4c855bbf33080ff3cc7eee)] update example in docs (Yoshua Wuyts)
- [[`0e954b7403`](https://github.com/yoshuawuyts/github_auth/commit/0e954b7403788a86aa4def0eb13fe681a23365c3)] (cargo-release) version 0.5.0 (Yoshua Wuyts)
- [[`a2a2d7ab96`](https://github.com/yoshuawuyts/github_auth/commit/a2a2d7ab9674df1af414e9b5c9b533f839c87395)] fix leaky struct (Yoshua Wuyts)
- [[`b54470ec61`](https://github.com/yoshuawuyts/github_auth/commit/b54470ec61e0b55ab17902e1a0fc803e7b871768)] (cargo-release) start next development iteration 0.4.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 Cargo.lock           |   2 +-
 Cargo.toml           |   2 +-
 README.md            |  40 +++++++++++++-
 examples/main.rs     |  14 +++--
 src/authenticator.rs | 135 +++++++++++++++++++++++++++++++++++++++++++++++-
 src/builder.rs       |  47 ++++++++++++++++-
 src/lib.rs           | 150 +++-------------------------------------------------
 src/scopes.rs        | 105 ++++++++++++++++++++++++++++++++++++-
 src/token.rs         |  10 +++-
 9 files changed, 357 insertions(+), 148 deletions(-)
```


