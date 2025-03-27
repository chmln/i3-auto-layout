# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.3.8](https://github.com/pando85/i3-auto-layout/tree/v0.3.8) - 2025-03-27

### Build

- Update Rust crate anyhow to v1.0.96
- Update Rust crate anyhow to v1.0.97
- Update Rust crate tokio to v1.44.0
- Update Rust crate log to v0.4.26
- Update KSXGitHub/github-actions-deploy-aur action to v4
- Update Rust crate tokio to v1.44.1
- Update to 2024 edition
- Add sha256sum to release binaries

## [v0.3.7](https://github.com/pando85/i3-auto-layout/tree/v0.3.7) - 2025-01-24

### Fixed

- Clippy Github Action name typo
- Remove `token` deprecated attr from `rs-clippy-check`

### Build

- Update Rust crate flexi_logger to v0.29.3
- Update Rust crate anyhow to v1.0.90
- Update Rust crate anyhow to v1.0.91
- Update Rust crate anyhow to v1.0.92
- Update Rust crate anyhow to v1.0.93
- Update Rust crate anyhow to v1.0.94
- Update Rust crate anyhow to v1.0.95
- Update Rust crate log to v0.4.25
- Update clechasseur/rs-clippy-check action to v4
- Update Rust crate tokio to v1.43.0
- Update wagoid/commitlint-github-action action to v6.2.1
- Update Rust crate tokio-stream to v0.1.17
- Update Rust crate flexi_logger to v0.29.8

## [v0.3.6](https://github.com/pando85/i3-auto-layout/tree/v0.3.6) - 2024-10-06

### Added

- Add release script

### Documentation

- Order changelog groups

### Build

- Update Rust crate tokio to v1.39.3
- Update Rust crate flexi_logger to 0.29.0
- Update KSXGitHub/github-actions-deploy-aur action to v3
- Update wagoid/commitlint-github-action action to v6.1.1
- Update wagoid/commitlint-github-action action to v6.1.2
- Update Rust crate anyhow to v1.0.89
- Remove pinned versions from `Cargo.toml`
- Optimize release binary
- Update Rust crate flexi_logger to v0.29.1
- Update Rust crate tokio to v1.40.0
- Update Rust crate tokio-stream to v0.1.16
- Change clippy to clechasseur/rs-clippy-check action to v3

## [v0.3.5](https://github.com/pando85/i3-auto-layout/tree/v0.3.5) - 2024-08-07

### Build

- Update Rust crate flexi_logger to v0.28.5
- Update KSXGitHub/github-actions-deploy-aur action to v2.7.2
- Update Rust crate log to v0.4.22
- Update wagoid/commitlint-github-action action to v6.0.2
- Update Rust crate tokio to v1.39.2

### CI

- Update build base to Ubuntu 20.04

### Fixed

- Handle all i3 events without breaking the loop

## [v0.3.4](https://github.com/pando85/i3-auto-layout/tree/v0.3.4) - 2024-06-24

### CI

- Add autotag workflow

## [v0.3.3](https://github.com/pando85/i3-auto-layout/tree/v0.3.3) - 2024-06-21

### Build

- Update Rust crate flexi_logger to 0.28.0
- Update wagoid/commitlint-github-action action to v6.0.1
- Update Rust crate flexi_logger to v0.28.4
- Update Rust crate tokio to v1.38.0
- Update Rust crate anyhow to v1.0.86

### CI

- Add automerge for renovate patch versions

### Fixed

- Change flexi logger to UTC

## [v0.3.2](https://github.com/pando85/i3-auto-layout/tree/v0.3.2) - 2024-05-15

### Added

- add timestamp to log

### Fixed

- Recreate channel when channel closed

### Build

- Update KSXGitHub/github-actions-deploy-aur action to v2.7.1

### Documentation

- Update release test copy dir

## [v0.3.1](https://github.com/pando85/i3-auto-layout/tree/v0.3.1) - 2024-05-09

### CI

- Remove mac release

### Documentation

- Fix release workflow order

### Fixed

- Parse version correctly in aur action

## [v0.3.0](https://github.com/pando85/i3-auto-layout/tree/v0.3.0) - 2024-05-09

### Core

- Add CI, doc and update dependencies
