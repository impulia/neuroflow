# Publishing Guide

This document outlines the automated process for releasing and publishing new versions of Neflo.

## Automated Versioning and Releases

Neflo uses [release-plz](https://github.com/MarcoIeni/release-plz) to automate the release process. Versioning is based on [Semantic Versioning (SemVer)](https://semver.org/) and is inferred from [Conventional Commits](https://www.conventionalcommits.org/).

### How it works

1. **Merge to Main**: When a Pull Request is merged into the `main` branch, a GitHub Action is triggered.
2. **Version Detection**: The system analyzes commit messages since the last release to determine the version bump (Major, Minor, or Patch).
3. **Cargo.toml & Changelog**: The version in `src-tauri/Cargo.toml` and `CHANGELOG.md` are updated automatically.
4. **GitHub Release**: A new Git tag is created and a GitHub Release is published.
5. **Build Assets**: macOS binaries (Intel and Apple Silicon) are built and attached to the release.

## Conventional Commits

All Pull Request titles must follow the Conventional Commits format:

- `feat: ...` for new features (Minor version bump).
- `fix: ...` for bug fixes (Patch version bump).
- `chore:`, `docs:`, `refactor:`, `style:`, `test:`, `ci:` for non-version-affecting changes.
- `BREAKING CHANGE:` or `!` after the type triggers a Major version bump.

This is enforced by the `mergeable` check on all Pull Requests.

## Manual Steps (Rare)

Under normal circumstances, no manual steps are required. The system is designed to be "hands-off."

If you need to manually trigger a release or fix a failed one, check the "Release" workflow in the GitHub Actions tab.

---

[Home](index.md) | [Previous: Development](development.md)
