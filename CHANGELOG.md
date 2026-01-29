# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Add `--flavor`/`-F` CLI flag for markdown flavor selection (mdbook, docusaurus)
  ([#4](https://github.com/o1-labs/cargo-specification/pull/4))
- Add `[output]` config section with optional `flavor` field
  ([#4](https://github.com/o1-labs/cargo-specification/pull/4))
- Docusaurus flavor transforms: admonitions (`:::warning`), math align
  (`\begin{aligned}`), TOC marker removal, math underscore escaping
  ([#4](https://github.com/o1-labs/cargo-specification/pull/4))

### Changed

- Use nightly rustfmt with mina-rust config (imports_granularity = "Crate")
  ([#4](https://github.com/o1-labs/cargo-specification/pull/4))
- Updated all dependencies to latest versions (askama 0.15.4, clap 4.5.55,
  comrak 0.50.0, miette 7.6.0, notify 8.2.0, serde 1.0.228, thiserror 2.0.18,
  tinytemplate 1.2.1, toml 0.9.11)
  ([#3](https://github.com/o1-labs/cargo-specification/pull/3))

## [0.5.0] - 2023-02-16

- Added a library that can be used directly (for example, in `build.rs` files)

## [0.4.4] - 2022-08-10

- Added support for paths starting with `@/` (only works in local git repositories)

## [0.4.3] - 2021-06-20

- Binary releases should work now.

## [0.4.2] - 2021-06-20

- Creating binary releases.
- Adding `CHANGELOG.md`.
- Upgrading comrek to 0.13.0.
