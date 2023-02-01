# Release

## Setup

Install cargo tools:

```sh
cargo install cargo-dist
cargo install cargo-release
```

Install GitHub CLI, which we use to build and release with version tags, optimized settings, and binary artifacts.

Example using macOS and brew package manager:

```sh
brew install gh
```

You may want to look at the file `.github/workflows/release.yml` because it describes the release process.
