# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## [v0.14.4] - 2023-11-15

- Bump svd-rs dependency to 0.14.4 or higher.

## [v0.14.3] - 2023-11-15

- Correctly place `expand_properties` under `expand` feature

## [v0.14.2] - 2023-09-17

- Bump MSRV to 1.58.0
- Ignore whitespaces around tag contents

## [v0.14.1] - 2022-10-23

- Update to `svd-rs` 0.14.1
- Add `BlockPath::parent`

## [v0.14.0] - 2022-07-19

- Make `expand::Index`, pathes & `derive_peripheral`, etc. public
- Fix parsing `xs:noNamespaceSchemaLocation`
- Bump MSRV to 1.56.0 (2021)

## [v0.13.4] - 2022-05-13

- Support nested `derivedFrom` for `expand`

## [v0.13.3] - 2022-05-09

- Add `expand_properties` (under `expand` feature)

## [v0.13.2] - 2022-04-23

- Add `expand` (under `expand` feature) and `ignore_enums` options

## [v0.13.1] - 2022-01-04

- Make `version`, `description`, `width` and `address_unit_bits` on `Device` optional again

## [v0.13.0] - 2022-01-04

- Add `svd2yaml` example
- Bump `svd-rs`

## [v0.12.0] - 2021-11-11

- Bump `svd-rs`
- Add `protection` parsing
- Add `readAction` parsing
- Add array support for peripherals

## [v0.11.0] - 2021-10-02

Previous versions in common [changelog](../CHANGELOG.md).

[Unreleased]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.4...HEAD
[v0.14.4]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.3...svd-parser-v0.14.4
[v0.14.3]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.2...svd-parser-v0.14.3
[v0.14.2]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.2...svd-parser-v0.14.2
[v0.14.1]: https://github.com/rust-embedded/svd/compare/v0.14.0...svd-rs-v0.14.1
[v0.14.0]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.4...v0.14.0
[v0.13.4]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.3...svd-parser-v0.13.4
[v0.13.3]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.2...svd-parser-v0.13.3
[v0.13.2]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.13.2...svd-parser-v0.13.2
[v0.13.1]: https://github.com/rust-embedded/svd/compare/v0.13.0...svd-parser-v0.13.1
[v0.13.0]: https://github.com/rust-embedded/svd/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/rust-embedded/svd/compare/v0.11.0...v0.12.0
[v0.11.0]: https://github.com/rust-embedded/svd/compare/v0.10.2...v0.11.0
