//! Lambek is a library that enables type-level programming in _stable_ Rust,
//! supporting advanced features including higher kinded types,
//! higher ranked types, and constraint kinds. Although Rust do not natively
//! support these features, Lambek uses techniques including _defunctionalization_
//! and _CPS transformation_ to emulate these features in Rust.
//!
//! Learn more about Lambek on the project
//! [GitHub page](https://github.com/maybevoid/lambek).

#[warn(clippy::all)]
#[macro_use]
pub mod type_app;

/// Traits for the kind of binary type application, `Type -> Type -> Type`.
pub mod bi_type_app;

/// Traits for constraint kinds, `Type -> Constraint`
pub mod constraint;

/// The standard `Functor`, `Applicative`, and `Monad` traits.
pub mod functor;

/// Traits for implementing extensible products and variants
pub mod row;

/// Extensible Variants
pub mod sum;

/// Extensible Products
pub mod product;

/// Natural Transformation, `type f ~> g = forall x. f x -> g x`
pub mod nat_trans;

#[cfg(test)]
pub mod test;
