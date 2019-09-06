# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- [breaking-change] `Defaults` field renamed on `RegisterProperties`
  and added into `Peripheral` and `ClusterInfo`
- Added `derived_from` into `Field` and `ClusterInfo`
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

[Unreleased]: https://github.com/japaric/svd/compare/v0.7.0...HEAD
[v0.7.0]: https://github.com/japaric/svd/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/japaric/svd/compare/v0.5.2...v0.6.0
[v0.5.2]: https://github.com/japaric/svd/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/japaric/svd/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/japaric/svd/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/japaric/svd/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/japaric/svd/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/svd/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/japaric/svd/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/svd/compare/v0.1.0...v0.1.1
