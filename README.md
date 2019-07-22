# vw-cdc-emulator.rs [![Build Status][travis.svg]][travis]

A mock implementation of a VW CD changer emulator in Rust.

Until [rust-lang/rust#44052][avr-rfc] lands, it's not possible to target AVR Arduino targets without a bunch of hacks.
The main reason for the existence of this repository is to do some data-modeling to better understand the actual
CD changer protocol in a high-level language.

## License

Licensed at your discretion under either:

 - [Apache Software License, Version 2.0](./LICENSE-APACHE)
 - [MIT License](./LICENSE-MIT)

 [avr-rfc]: https://github.com/rust-lang/rust/issues/44052
 [travis]: https://travis-ci.org/naftulikay/vw-cdc-emulator.rs
 [travis.svg]: https://travis-ci.org/naftulikay/vm-cdc-emulator.rs.svg?branch=master
