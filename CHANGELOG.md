# Changelog

All notable changes to this project will be documented in this file.

This project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0] - 2024-02-20

### Added

- `ErasedSet::type_ids`
- `ErasedSet::debug_type_names`, disabled in release mode,
  return an iterator over the names of the stored types.

### Changed

- MSRV changed from unspecified to 1.60

### Fixed

- Remove `must_use` attribute on `ErasedSet::get_or_insert` and
  `ErasedSet::get_or_insert_with`.

## [0.7.0] - 2022-05-30

### Removed

- Remove `ErasedSet::with_capacity`.
- Remove `ErasedSet::capacity`.
- Remove `ErasedSet::reserve`.
- Remove `ErasedSet::shrink_to`.
- Remove `ErasedSet::shrink_to_fit`.
- Remove `hashbrown` feature (`no_std` is still supported)

## [0.6.1] - 2022-05-28

### Added

- Add `ErasedSet::shrink_to`.
- Add `ErasedSet::get_or_insert`.
- Add `ErasedSet::get_or_insert_with`.

## [0.6.0] - 2022-05-11

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

[unreleased]: https://github.com/malobre/erased_set/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/malobre/erased_set/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/malobre/erased_set/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/malobre/erased_set/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/malobre/erased_set/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/malobre/erased_set/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/malobre/erased_set/releases/tag/v0.5.0
