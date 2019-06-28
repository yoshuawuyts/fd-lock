## 2019-06-28, Version 1.1.0
### Commits
- [[`0194679d1d`](https://github.com/yoshuawuyts/fd-lock/commit/0194679d1d2d48a8f7292d8f86ebd3a377456e21)] (cargo-release) version 1.1.0 (Yoshua Wuyts)
- [[`78c27fc11d`](https://github.com/yoshuawuyts/fd-lock/commit/78c27fc11d1a40cb78ee60585d3e711fa76e2100)] add lock (Yoshua Wuyts)
- [[`88b7a38409`](https://github.com/yoshuawuyts/fd-lock/commit/88b7a38409225e8fa6cec48b456a238c2db02b01)] readme example (Yoshua Wuyts)
- [[`a9b9f57279`](https://github.com/yoshuawuyts/fd-lock/commit/a9b9f5727923ca9e62c24ee965c7a4b160576740)] error name (Yoshua Wuyts)
- [[`253450102e`](https://github.com/yoshuawuyts/fd-lock/commit/253450102ea71c18a3cdbe1d14f059f35103b915)] try_lock & windows (Yoshua Wuyts)
- [[`3146ff4611`](https://github.com/yoshuawuyts/fd-lock/commit/3146ff46115c75e8ccf7d6d66c0fa9e354b42fc2)] locks! (Yoshua Wuyts)
- [[`1117084f3b`](https://github.com/yoshuawuyts/fd-lock/commit/1117084f3be3e951a858dade022fa70a1c12905b)] more (Yoshua Wuyts)
- [[`941b9a3b8c`](https://github.com/yoshuawuyts/fd-lock/commit/941b9a3b8c247a4554f27485c33e7331cdc7ea8e)] fmt (Yoshua Wuyts)
- [[`7c92773a79`](https://github.com/yoshuawuyts/fd-lock/commit/7c92773a796635b84119f3f5484d6c444cba94af)] more resources (Yoshua Wuyts)
- [[`8ede76becc`](https://github.com/yoshuawuyts/fd-lock/commit/8ede76becc38f9c50384a8c1fe81c9cdef124b6b)] . (Yoshua Wuyts)

### Stats
```diff
 .github/CODE_OF_CONDUCT.md                |  75 ++++++++++++-
 .github/CONTRIBUTING.md                   |  55 +++++++++-
 .github/ISSUE_TEMPLATE.md                 |   9 +-
 .github/ISSUE_TEMPLATE/bug_report.md      |  23 ++++-
 .github/ISSUE_TEMPLATE/feature_request.md |  43 +++++++-
 .github/ISSUE_TEMPLATE/question.md        |  18 +++-
 .github/PULL_REQUEST_TEMPLATE.md          |  14 ++-
 .github/stale.yml                         |  17 +++-
 .gitignore                                |   7 +-
 .travis.yml                               |  13 ++-
 Cargo.toml                                |  20 +++-
 LICENSE-APACHE                            | 190 +++++++++++++++++++++++++++++++-
 LICENSE-MIT                               |  21 +++-
 README.md                                 |  76 ++++++++++++-
 src/error.rs                              |  64 ++++++++++-
 src/lib.rs                                |  43 +++++++-
 src/unix.rs                               |  80 +++++++++++++-
 src/windows.rs                            | 100 ++++++++++++++++-
 tests/test.rs                             |   6 +-
 19 files changed, 874 insertions(+)
```


