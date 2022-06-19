## Check if given version is pre-release.

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Given two versions, the [`Prerelease`] field will be [`EMPTY`] if there is
not an optional pre-release identifier in the version string.

```rust
{{#include examples/semver-prerelease.rs}}
```

[`Prerelease`]: https://docs.rs/semver/latest/semver/struct.Prelease.html
[`EMPTY`]: https://docs.rs/semver/latest/semver/struct.Prerelease.html#associatedconstant.EMPTY
