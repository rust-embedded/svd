# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.10.2] - 2021-04-28

- Allow single valued `dimIndex`
- Added `reg_iter`, `reg_iter_mut` methods on `Peripheral` and `Cluster`
- Added `DerefMut` for `Cluster`, `Register` and `Field`
- Added `display_name` to `RegisterInfo`
- Added implementations of `From<Type>` for `TypeBuilder`'s

## [v0.10.1] - 2021-04-17

- Added `DeriveFrom` implementation for `FieldInfo`

## [v0.10.0] - 2021-04-04

- Added `strict` feature that hides part of checks
- Builder pattern now is used for creating structures
- Peripherals are processed in parallel with `rayon`
- Serde now skips empty tags
- Fix: bug in `addressUnitBits`
- Fix: panic with 32 bit wide fields that have enumerated values
- Fix: produce error on 0-width fields
- Fix: error instead of panic when an array/cluster name is missing the `%s` placeholder
- [breaking-change] Add support for 64 addresses, register values, enum values and writeConstraints
- [breaking-change] Remove unproven flag

## [v0.9.0] - 2019-11-17

- [breaking-change]  make `ClusterInfo` `description` optional
- [breaking-change]  use `anyhow` Error instead of `failure`

## [v0.8.1] - 2019-11-03

- Fix: make `derive_from` module public
- Fix: enumerated_values empty check

## [v0.8.0] - 2019-11-03

- [breaking-change]  `RegisterClusterArrayInfo` renamed on `DimElement`
- [breaking-change] `Defaults` field renamed on `RegisterProperties`
  and added into `Peripheral` and `ClusterInfo`
- [breaking-change] `Field` splitted on `Field` enum and `FieldInfo` struct
  to support field arrays
- Added `derived_from` into `Field` and `ClusterInfo`
- [breaking-change] implement `DeriveFrom` for `ClusterInfo`,
  `RegisterInfo` and `EnumeratedValues`
- Updated dependencies, use `Edition 2018`
- Added missing `zeroToToggle`
- Added serializing/deserializing with `serde`

## [v0.7.0] - 2019-01-11

- [breaking-change] Major Refactor
  - Split SVD components into modules
  - Improved error handling
  - Added `untested` encoding functions
  - Added a bunch of missing fields
- Added (and fixed) derivedFrom
- Made register description optional


## [v0.6.0] - 2018-02-24

### Changed

- Accept both 'X' and 'x' as "don't care" bits in literals.

- Parse clusters of registers

## [v0.5.2] - 2017-07-04

### Added

- A CPU field to the Device struct

## [v0.5.1] - 2017-04-29

### Added

- A WriteConstraint field to the RegisterInfo struct.

## [v0.5.0] - 2017-04-23

### Added

- [breaking-change] A WriteConstraint field to the Field struct.

### Changed

- [breaking-change]. Added a private field to Device, Peripheral, RegisterInfo,
  RegisterArrayInfo, Defaults, EnumeratedValues and EnumeratedValue to be able
  to add more fields to them in the future without technically breaking backward
  compatibility.

## [v0.4.0] - 2017-04-03

### Changed

- The type of `Peripheral.interrupt` changed from `Option<Interrupt>`
  to `Vec<Interrupt>` to properly reflect what the SVD allows.

## [v0.3.0] - 2017-02-18

### Changed

- The type of `Field.enumeratedValues` changed from `Option<EnumeratedValues>`
  to `Vec<EnumeratedValues>` to properly reflect what the SVD allows.

## [v0.2.0] - 2016-12-21

### Added

- Support for "register arrays". This converted the `struct Register` into an
  `enum` (to represent normal registers and register arrays) thus breaking
  construction of this item (which should be pretty rare).

## [v0.1.2] - 2016-12-07

### Added

- Implemented `Copy` and `Clone` for several structs

## [v0.1.1] - 2016-10-09

### Fixed

- the description of this crate

## v0.1.0 - 2016-10-09 [YANKED]

### Added

- Initial SVD parser
- A `parse` utility function to parse the contents of a SVD file (XML)

[Unreleased]: https://github.com/rust-embedded/svd/compare/v0.10.2...HEAD
[v0.10.2]: https://github.com/rust-embedded/svd/compare/v0.10.1...v0.10.2
[v0.10.1]: https://github.com/rust-embedded/svd/compare/v0.10.0...v0.10.1
[v0.10.0]: https://github.com/rust-embedded/svd/compare/v0.9.0...v0.10.0
[v0.9.0]: https://github.com/rust-embedded/svd/compare/v0.8.1...v0.9.0
[v0.8.1]: https://github.com/rust-embedded/svd/compare/v0.8.0...v0.8.1
[v0.8.0]: https://github.com/rust-embedded/svd/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/rust-embedded/svd/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/rust-embedded/svd/compare/v0.5.2...v0.6.0
[v0.5.2]: https://github.com/rust-embedded/svd/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/rust-embedded/svd/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/rust-embedded/svd/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/rust-embedded/svd/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/rust-embedded/svd/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/rust-embedded/svd/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/rust-embedded/svd/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/rust-embedded/svd/compare/v0.1.0...v0.1.1
