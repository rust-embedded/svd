# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

- Do not derive `DimElement`
- Revert the `riscv` elements, as well as the `unstable-riscv` feature.

## [v0.14.9] - 2024-08-20

- Add `riscv::Exception` for custom exception source enumerations.
- Add `riscv` element for configuration parameters related to RISC-V targets.
  You must use the `unstable-riscv` feature to enable this exeperimental element.
- Add `DataType`
- Fix run-time panic for write constraint check

## [v0.14.8] - 2024-02-13

- add `maybe_array` constructors
- Bump MSRV to 1.65.0
- Bump `regex` to 1.10

## [v0.14.7] - 2024-01-03

- Bump MSRV to 1.61.0
- use close range in `EnumeratedValue` error message

## [v0.14.6] - 2023-11-29

- fix `validate` on `Disabled` level, remove `mut`
- add `validate_all` for `Device` and childrens for recursive tree check

## [v0.14.5] - 2023-11-22

- `default_value` for `EnumeratedValues`
- moved `isDefault` and `value` check on `EnumeratedValue` to strict mode

## [v0.14.4] - 2023-11-15

- Add `expand` functions for arrays
- Fix `indexes_as_range`

## [v0.14.3] - 2023-10-24

- Bump MSRV to 1.58.0
- `array::names` returns custom names if specified
- add `Description` trait and `array::descriptions`
- add write constraint range check

## [v0.14.2] - 2023-05-10

- Add support of `a-Z` for `dimIndex`
- Add `name`, `description`, `address_offset` for `RegisterCluster`

## [v0.14.1] - 2022-10-23

- (De)serialize `dimIndex` (from)to string

## [v0.14.0] - 2022-07-19

- Bump MSRV to 1.56.0 (2021)

## [v0.13.2] - 2022-04-12

- Fix `schema_version` deserialization

## [v0.13.1] - 2022-02-12

- add `indexes_as_range` for `DimElement`

## [v0.13.0] - 2022-01-04

- fixed `BitRange` deserializing
- skip serializing optional fields in `Cpu` if empty
- skip serializing `values` in `EnumeratedValues` if empty
- add `names` function for arrays, `base_addresses` for `Peripheral`,
  `address_offsets` for `Register` and `Cluster`, `bit_offsets` for `Field` arrays
- add missing fields in `Device`, require `version`, `description`, `address_unit_bits` and `width`,
  also `schema_version` is required, but skipped during (de)serialization
- merge `register` with `registerinfo` modules same as other `info`s
- camelCase for WriteConstraint serialization
- `EnumeratedValues.usage()` now return `None` if it is derived, fix bug in usage check
- Use generic `MaybeArray` enum for types which can be either collected into SVD arrays or have only one instance
- `Name` trait for structures that has `name` field
- improves in iterators
- `get_enumerated_values` by usage

## [v0.12.1] - 2021-12-08

- Rename `reg_iter` to `all_registers`,
- Add `registers`, `clusters`, `fields` methods that create iterators
- Add `get_register`, `get_cluster`, `get_field` (also `mut`) which take child by name

## [v0.12.0] - 2021-11-11

- Bump dependencies
- Add check for wrong size of `bitRange` width
- Don't clone when serialize
- Add optional entries to `Cpu`
- `AddressBlock` & `Interrupt` now use builders
- Add `dim_name` and `dim_array_index` to `DimElement`
- Add `alternate_peripheral`, `prepend_to_name`, `append_to_name`,
  `header_struct_name` to `PeripheralInfo`, `alternate_cluster` to `ClusterInfo`
- Add `protection` to `RegisterProperties` and `AddressBlock`
- Add `readAction` to `RegisterInfo` and `FieldInfo`
- Add `single` and `array` for `Info` types,
  `is_single` and `is_array` for `Peripheral`, `Cluster`, `Register` and `Field`
- Add array support for peripherals

## [v0.11.2] - 2021-11-04

- Implement `DeriveFrom` for `Cluster`, `Register` and `Field`

## [v0.11.1] - 2021-10-02

- Reexport builders
- Fix typo in Access::can_write

## [v0.11.0] - 2021-10-02
- Splitted from `svd-parser`

Previous versions in common [changelog](../CHANGELOG.md).

[Unreleased]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.9...HEAD
[v0.14.9]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.8...svd-rs-v0.14.9
[v0.14.8]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.7...svd-rs-v0.14.8
[v0.14.7]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.6...svd-rs-v0.14.7
[v0.14.6]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.5...svd-rs-v0.14.6
[v0.14.5]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.4...svd-rs-v0.14.5
[v0.14.4]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.3...svd-rs-v0.14.4
[v0.14.3]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.2...svd-rs-v0.14.3
[v0.14.2]: https://github.com/rust-embedded/svd/compare/svd-encoder-v0.14.2...svd-rs-v0.14.2
[v0.14.1]: https://github.com/rust-embedded/svd/compare/v0.14.0...svd-rs-v0.14.1
[v0.14.0]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.13.2...v0.14.0
[v0.13.2]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.13.1...svd-rs-v0.13.2
[v0.13.1]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.1...svd-rs-v0.13.1
[v0.13.0]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.12.1...v0.13.0
[v0.12.1]: https://github.com/rust-embedded/svd/compare/v0.12.0...svd-rs-v0.12.1
[v0.12.0]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.11.2...v0.12.0
[v0.11.2]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.11.1...svd-rs-v0.11.2
[v0.11.1]: https://github.com/rust-embedded/svd/compare/v0.11.0...svd-rs-v0.11.1
[v0.11.0]: https://github.com/rust-embedded/svd/compare/v0.10.2...v0.11.0
