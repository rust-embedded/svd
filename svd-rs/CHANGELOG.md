# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

- skip serializing `values` in `EnumeratedValues` if empty
- add missing fields in `Device`, require `version`, `description`, `address_unit_bits` and `width`,
  also `schema_version` is required, but skipped during (de)serialization
- merge `register` with `registerinfo` modules same as other `info`s
- `EnumeratedValues.usage()` now return `None` if it is derived, fix bug in usage check
- Use generic `SingleArray` enum for types which can be either collected into SVD arrays or have only one instance
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

Previous versions in common [changelog][../CHANGELOG.md].

