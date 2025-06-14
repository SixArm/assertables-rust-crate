# Releasing

Releasing a new version needs a few steps that we want to automate, yet are currently manual:

## Update the version in file `Cargo.toml`

Edit the file `Cargo.toml`.

Update the version number.

## Update the version and date in the files `lib.rs` and `README.md`

Edit the file `lib.rs` and `README.md`.

Update the version number.

Update the date stamp, which uses ISO UTC format.

## Update the version in all source files

Search for:

```text
assertables/1.0.0/
```

Replace with:

```text
assertables/1.1.0/
```

## Verify

Verify everything is correct locally:

```sh
cargo fmt
cargo test
cargo build --release
cargo doc
```

## Release

Releasing a new version uses these steps:

```sh
git commit "Your message here"
git tag "1.1.1"
git push --tags
cargo publish
```

Confirm that the crate is published:

* <https://crates.io/crates/assertables>
