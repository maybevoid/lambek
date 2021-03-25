//! Traits for the kind of unary type application, `Type -> Type`.
//!
//! Higher kinded types (HKT) such as `Type -> Type` are not natively
//! supported in Rust. As such, we cannot use type constructors
//! such as `Vec` without applying a specific type as an argument,
//! e.g. `Vec<u8>`. Although the upcoming generic associated types (GAT)
//! feature will partially solve this issue, the feature is not yet
//! stable and may subject to changes.
//!
//! An alternative approach is to use _defunctionalization_ to encode
//! regular Rust types to have kinds other than `Type`. [TypeApp]
//! is one such trait for encoding types of kind `Type -> Type`.
//! For a practical example, refer to [VecF].

use std::marker::PhantomData;

/// A type `F` implements `TypeCon` to mark itself as having the kind
/// `Type -> Type`.
///
/// Although the requirement is non-binding, types
/// that implement `TypeCon` are also expected to implement [TypeApp].
/// For stronger guarantee that a type `F` really implements
/// [TypeApp] for all type arguments, use [TypeAppGeneric] instead.
///
/// In practice, it is usually sufficient to require type constructors
/// to implement just [TypeCon]. This is because the constraint for
/// [TypeAppGeneric] may sometimes be too strict, i.e. we may
/// want to allow types that implement [TypeApp] for some
/// constrained type arguments such as `Send` or `'static`.
pub trait TypeCon
{
}

pub trait TypeApp<'a, X : 'a + ?Sized>: TypeCon
{
  type Applied: 'a + ?Sized;
}

pub trait TypeAppGeneric: TypeCon + Sized
{
  fn with_type_app<'a, X : 'a, R : 'a>(
    cont : impl TypeAppCont<'a, Self, X, R>
  ) -> R
  where
    Self : 'a;
}

pub trait TypeAppCont<'a, F : 'a, X : 'a, R : 'a>
{
  fn on_type_app(self) -> R
  where
    F : TypeApp<'a, X>;
}

pub trait HasTypeApp<'a, F : 'a + ?Sized, X : 'a + ?Sized>
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F : TypeApp<'a, X>;

  fn get_applied_borrow(&self) -> &F::Applied
  where
    F : TypeApp<'a, X>;

  fn get_applied_borrow_mut(&mut self) -> &mut F::Applied
  where
    F : TypeApp<'a, X>;
}

pub type App<'a, F, X> = Box<dyn HasTypeApp<'a, F, X> + 'a>;

pub fn wrap_app<'a, F : 'a, X : 'a, FX : 'a>(fx : FX) -> App<'a, F, X>
where
  F : TypeApp<'a, X, Applied = FX>,
{
  Box::new(fx)
}

impl<'a, F : 'a, X : 'a, FX : 'a> HasTypeApp<'a, F, X> for FX
where
  F : TypeApp<'a, X, Applied = FX>,
{
  fn get_applied(self: Box<Self>) -> Box<FX>
  {
    self
  }

  fn get_applied_borrow(&self) -> &FX
  {
    self
  }

  fn get_applied_borrow_mut(&mut self) -> &mut FX
  {
    self
  }
}

pub enum Identity {}

impl TypeCon for Identity {}

impl TypeAppGeneric for Identity
{
  fn with_type_app<'a, X : 'a, R : 'a>(
    cont : impl TypeAppCont<'a, Self, X, R>
  ) -> R
  where
    Self : 'a,
  {
    cont.on_type_app()
  }
}

impl<'a, X : 'a + ?Sized> TypeApp<'a, X> for Identity
{
  type Applied = X;
}

pub struct Const<A : ?Sized>(PhantomData<A>);

impl<A : ?Sized> TypeCon for Const<A> {}

impl<'a, A : 'a + ?Sized, X : 'a + ?Sized> TypeApp<'a, X> for Const<A>
{
  type Applied = A;
}

pub enum Borrow {}

impl TypeCon for Borrow {}

impl<'a, X : 'a + ?Sized> TypeApp<'a, X> for Borrow
{
  type Applied = &'a X;
}

impl TypeAppGeneric for Borrow
{
  fn with_type_app<'a, X : 'a, R : 'a>(
    cont : impl TypeAppCont<'a, Self, X, R>
  ) -> R
  where
    Self : 'a,
  {
    cont.on_type_app()
  }
}

pub enum BorrowMut {}

impl TypeCon for BorrowMut {}

impl<'a, X : 'a + ?Sized> TypeApp<'a, X> for BorrowMut
{
  type Applied = &'a mut X;
}

impl TypeAppGeneric for BorrowMut
{
  fn with_type_app<'a, X : 'a, R : 'a>(
    cont : impl TypeAppCont<'a, Self, X, R>
  ) -> R
  where
    Self : 'a,
  {
    cont.on_type_app()
  }
}

pub enum VecF {}

impl TypeCon for VecF {}

impl < 'a, X: 'a > TypeApp < 'a, X > for VecF {
  type Applied = Vec < X >;
}

impl TypeAppGeneric for VecF
{
  fn with_type_app<'a, X : 'a, R : 'a>(
    cont : impl TypeAppCont<'a, Self, X, R>
  ) -> R
  where
    Self : 'a,
  {
    cont.on_type_app()
  }
}
