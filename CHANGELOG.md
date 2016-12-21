# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/japaric/svd/compare/v0.1.2...HEAD
[v0.1.2]: https://github.com/japaric/svd/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/svd/compare/v0.1.0...v0.1.1
