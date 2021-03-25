# Lambek: Type-Level Programming in Rust

Lambek is a library that enables type-level programming in _stable_ Rust,
supporting advanced features including higher kinded types,
higher ranked types, and constraint kinds. Although Rust do not natively
support these features, Lambek uses techniques including _defunctionalization_
and _CPS transformation_ to emulate these features in Rust.

Lambek is currently in _early development_ phase. The techniques presented
in Lambek are from the author's experience in developing
[Ferrite](https://github.com/ferrite-rs/ferrite).
