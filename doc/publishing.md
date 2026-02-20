# Publishing Guide

This document outlines the process for releasing and publishing new versions of Neflo.

## Versioning

Neflo follows [Semantic Versioning (SemVer)](https://semver.org/).
- Update the version in `Cargo.toml` before a release.

## Release Checklist

1. **Verify Tests**: Ensure all tests pass on a clean environment.
   ```bash
   cargo test
   ```
2. **Linting**: Run clippy and ensure there are no warnings.
   ```bash
   cargo clippy -- -D warnings
   ```
3. **Formatting**: Ensure code is formatted.
   ```bash
   cargo fmt -- --check
   ```
4. **Update Changelog**: (Optional) Update a CHANGELOG.md file with notable changes.
5. **Tag the Release**: Create a git tag for the version.
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

## Publishing to Crates.io

To publish the package to the official Rust package registry:

1. **Login**: Ensure you are authenticated with `cargo login`.
2. **Dry Run**: Check if the package is ready for publishing.
   ```bash
   cargo publish --dry-run
   ```
3. **Publish**:
   ```bash
   cargo publish
   ```

## GitHub Actions

The project uses GitHub Actions to automatically run tests and linters on every push and pull request. Ensure the CI pipeline is green before merging any changes to the main branch.

---

[Home](index.md) | [Previous: Development](development.md)
