# Publishing Guide

This document outlines the automated process for releasing and publishing new versions of Neflo.

## Automated Versioning and Releases

Neflo uses [release-plz](https://github.com/MarcoIeni/release-plz) to automate the release process. Versioning is based on [Semantic Versioning (SemVer)](https://semver.org/) and is inferred from [Conventional Commits](https://www.conventionalcommits.org/).

### How it works

1.  **Merge to Main**: When a Pull Request is merged into the `main` branch, a GitHub Action is triggered.
2.  **Version Detection**: The system analyzes the commit messages since the last release to determine if a Major, Minor, or Patch version bump is required.
3.  **Cargo.toml & Changelog**: The system automatically updates the version in `Cargo.toml` and appends the changes to `CHANGELOG.md`.
4.  **GitHub Release**: A new Git tag is created, and a GitHub Release is published with the generated changelog.
5.  **Build Assets**: For every release, a macOS binary is automatically built and attached to the GitHub Release as an asset.
6.  **Crates.io**: If the `CARGO_REGISTRY_TOKEN` secret is configured in the repository, the new version is automatically published to [crates.io](https://crates.io/).

## Conventional Commits

To ensure the automated system works correctly, all Pull Request titles must follow the Conventional Commits format:

-   `feat: ...` for new features (triggers a Minor version bump).
-   `fix: ...` for bug fixes (triggers a Patch version bump).
-   `chore:`, `docs:`, `refactor:`, `style:`, `test:`, `ci:` for changes that don't affect the version or trigger a Patch bump depending on configuration.
-   Adding `!` after the type or `BREAKING CHANGE:` in the footer triggers a Major version bump.

This is enforced by the `mergeable` check on all Pull Requests.

## Manual Steps (Rare)

Under normal circumstances, no manual steps are required for a release. The system is designed to be "hands-off."

If you need to manually trigger a release or fix a failed release, you can check the "Release" workflow in the GitHub Actions tab.

---

[Home](index.md) | [Previous: Development](development.md)
