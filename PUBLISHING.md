# Publishing Guide

This document explains how to publish this crate to crates.io.

## Prerequisites

1. Rust installed (https://rustup.rs/)
2. A crates.io account (https://crates.io/me)
3. Your API token configured in `~/.cargo/credentials.toml`

## Steps to Publish

### 1. Verify Everything Works

```bash
cargo test
cargo test --release
cargo clippy
cargo fmt
```

### 2. Update Version

Edit `Cargo.toml` and update the version number following [Semantic Versioning](https://semver.org/):

```toml
[package]
version = "0.2.0"  # Change this
```

### 3. Update Changelog

Add an entry to `CHANGELOG.md` with:
- Version number
- Release date
- Summary of changes

### 4. Verify Documentation

```bash
cargo doc --no-deps --open
```

Ensure all public APIs are documented and examples work correctly.

### 5. Dry Run

Test the publishing process without uploading:

```bash
cargo publish --dry-run
```

### 6. Publish

When ready, publish to crates.io:

```bash
cargo publish
```

### 7. Verify

Visit https://crates.io/crates/tajweed_warsh_rules to confirm publication.

## Using the Published Crate

Once published, users can add it to their `Cargo.toml`:

```toml
[dependencies]
tajweed_warsh_rules = "0.2"
```

## Best Practices

1. **Semantic Versioning**: Follow SemVer for version bumps
   - MAJOR: Breaking changes
   - MINOR: New features (backward compatible)
   - PATCH: Bug fixes (backward compatible)

2. **Documentation**: Ensure all public items are documented
3. **Tests**: Run tests before publishing
4. **Changelog**: Keep changelog up-to-date
5. **Tags**: Consider creating git tags for releases

## Yanking Versions

If you need to remove a published version:

```bash
cargo yank --vers 0.2.0
```

This prevents new users from downloading the version but keeps existing dependencies intact.

## Repository Setup

After initial publication, update the Cargo.toml with actual repository information:

```toml
[package]
repository = "https://github.com/m97chahboun/tajweed_warsh_rules"
documentation = "https://docs.rs/tajweed_warsh_rules"
```

## Issues?

If you encounter issues:

1. Check crates.io documentation: https://doc.rust-lang.org/cargo/
2. Review your crate's documentation on docs.rs
3. Check for any validation errors in the cargo output
