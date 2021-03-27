# Lambek: Type-Level Programming in Rust

[![Crates.io][crates-badge]][crates-url]
[![Documentation][doc-badge]][doc-url]
[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/lambek.svg
[crates-url]: https://crates.io/crates/lambek
[doc-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[doc-url]: https://maybevoid.com/lambek-doc/lambek/
[license-badge]: https://img.shields.io/crates/l/lambek.svg
[license-url]: https://github.com/maybevoid/lambek/blob/master/LICENSE
[actions-badge]: https://github.com/maybevoid/lambek/workflows/Cargo%20Tests/badge.svg
[actions-url]: https://github.com/maybevoid/lambek/actions


Lambek is a library that enables type-level programming in _stable_ Rust,
supporting advanced features including higher kinded types,
higher ranked types, and constraint kinds. Although Rust do not natively
support these features, Lambek uses techniques including _defunctionalization_
and _CPS transformation_ to emulate these features in Rust.

Lambek is currently in _early development_ phase. The techniques presented
in Lambek are from the author's experience in developing
[Ferrite](https://github.com/ferrite-rs/ferrite).
