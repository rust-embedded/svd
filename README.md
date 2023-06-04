# Rust SVD manipulating libraries

This project is developed and maintained by the [Tools team][team].

It consists of:

- [`svd-rs`](https://docs.rs/svd-rs). Basic structures and builders, also (de)serializers under `serde` feature.
- [`svd-parser`](https://docs.rs/svd-parser). Library for parsing SVD XML source in Rust `Device` structure.
- [`svd-encoder`](https://docs.rs/svd-encoder). Library for creating SVD XML.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.58.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Tools team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-tools-team
