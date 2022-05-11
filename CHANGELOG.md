# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Rename crate to `erased_set`.
- Rename `StaticTypeMap` to `ErasedSet`.
- Rename `SendStaticTypeMap` to `ErasedSendSet`.
- Rename `SendSyncStaticTypeMap` to `ErasedSyncSet`.

## [0.5.1] - 2022-05-08

## [0.5.0] - 2022-05-08

### Changed
- `hashbrown` feature replaces `no_std`.

### Added
- `Debug` implementations.

## [0.4.1] - 2022-02-06

[unreleased]: https://github.com/malobre/erased_set/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/malobre/erased_set/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/malobre/erased_set/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/malobre/erased_set/releases/tag/v0.4.1
