//! Traits for the kind of unary type application, `Type -> Type`.
//!
//! Higher kinded types (HKT) such as `Type -> Type` are not natively
//! supported in Rust. As such, we cannot use type constructors
//! such as `Vec` without applying a specific type as an argument,
//! e.g. `Vec<u8>`. Although the upcoming generic associated types (GAT)
//! feature will partially solve this issue, the feature is not yet
//! stable and may subject to changes.
//! An alternative approach is to use _defunctionalization_ to encode
//! regular Rust types to have kinds other than `Type`. [TypeApp]
//! is one such trait for encoding types of kind `Type -> Type`.
//!
//! To promote a type constructor such as [Vec] to HKT, we define a
//! proxy type [VecF] and implement [TypeCon] and [TypeApp] for them.
//! We then use `VecF` as the unapplied version of `Vec` in our code.
//! Inside type signatures, we use `App<VecF, X>` to apply `VecF`
//! to a type `X`. `App<VecF, X>` is isomorphic to `Vec<X>`, and
//! we can convert a value `xs: App<VecF, X>` back into `Vec<X>`
//! by calling `xs.get_applied()`.
//!
//! The type [App] encapsulates the [TypeApp] constraint using
//! [HasTypeApp]. With that, type signatures that use `App<F, X>`
//! do not need to have `TypeApp<F, X>` in their trait bounds.
//! This makes it significantly easier to perform arbitrary type
//! applications without having to worry about polluting the
//! trait bounds with `TypeApp` constraints. See
//! [Functor](crate::functor::Functor) for a practical use of [App].

use std::{
  any::Any,
  marker::PhantomData,
};

/// A proxy type `F` implements `TypeCon` to mark itself as having the kind
/// `Type -> Type`.
///
/// The type `F` itself is never used directly, so it don't need to have
/// any inhabitant and may be declared as an empty enum.
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
  fn with_type_app<'a, X : 'a, R : 'a, Cont : 'a>(cont : Box<Cont>) -> R
  where
    Self : 'a,
    Cont : TypeAppCont<'a, Self, X, R>;
}

pub trait TypeAppCont<'a, F : 'a, X : 'a, R : 'a>
{
  fn on_type_app(self: Box<Self>) -> R
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

  fn with_type_app<'b>(
    &'b self,
    cont : Box<dyn TypeAppCont<'a, F, X, Box<dyn Any + 'b>>>,
  ) -> Box<dyn Any + 'b>;
}

pub type App<'a, F, X> = Box<dyn HasTypeApp<'a, F, X> + 'a>;

pub fn wrap_app<'a, F : 'a, X : 'a, FX : 'a>(fx : FX) -> App<'a, F, X>
where
  F : TypeApp<'a, X, Applied = FX>,
{
  Box::new(fx)
}

#[macro_export]
macro_rules! define_type_app {
  ( $proxy:ident, $target:ident ) => {
    pub enum $proxy {}
    $crate::impl_type_app!($proxy, $target);
  };
  ( $proxy:ident < $( $types:ident ),+ $(,)? >, $target:ident ) => {
    #[allow(unused_parens)]
    pub struct $proxy < $( $types ),* >
      ( std::marker::PhantomData< ( $( $types ),* ) > );

    $crate::impl_type_app!($proxy <$( $types ),* >, $target);
  };
}

#[macro_export]
macro_rules! impl_type_app {
  ( $proxy:ident, $target:ident ) => {
    impl TypeCon for $proxy {}

    impl < 'a, X: 'a > TypeApp < 'a, X > for $proxy {
      type Applied = $target < X >;
    }

    impl TypeAppGeneric for $proxy
    {
      fn with_type_app<'a, X : 'a, R : 'a, Cont: 'a>(
        cont : Box < Cont >
      ) -> R
      where
        Self : 'a,
        Cont: TypeAppCont<'a, Self, X, R>,
      {
        cont.on_type_app()
      }
    }
  };
  ( $proxy:ident < $( $types:ident ),+ $(,)? >, $target:ident ) => {
    impl < $( $types ),* >
      TypeCon for $proxy < $( $types ),* > {}

    impl < 'a, X: 'a, $( $types : 'a ),* >
      TypeApp < 'a, X > for $proxy < $( $types ),* >
    {
      type Applied = $target < $( $types ),*, X >;
    }

    impl < $( $types ),* >
      TypeAppGeneric for $proxy < $( $types ),* >
    {
      fn with_type_app<'a, X : 'a, R : 'a, Cont: 'a>(
        cont : Box < Cont >
      ) -> R
      where
        Self : 'a,
        Cont: TypeAppCont<'a, Self, X, R>,
      {
        cont.on_type_app()
      }
    }
  }
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

  fn with_type_app<'b>(
    &'b self,
    cont : Box<dyn TypeAppCont<'a, F, X, Box<dyn Any + 'b>>>,
  ) -> Box<dyn Any + 'b>
  {
    cont.on_type_app()
  }
}

/// `App<Identity, X> ~ X`
pub enum Identity {}

impl TypeCon for Identity {}

impl<'a, X : 'a + ?Sized> TypeApp<'a, X> for Identity
{
  type Applied = X;
}

impl TypeAppGeneric for Identity
{
  fn with_type_app<'a, X : 'a, R : 'a, Cont : 'a>(cont : Box<Cont>) -> R
  where
    Self : 'a,
    Cont : TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

/// `App<Const<A>, X> ~ A`
pub struct Const<A : ?Sized>(PhantomData<A>);

impl<A : ?Sized> TypeCon for Const<A> {}

impl<'a, A : 'a + ?Sized, X : 'a + ?Sized> TypeApp<'a, X> for Const<A>
{
  type Applied = A;
}

/// `App<'a, Borrow, X> ~ &'a X`
pub enum Borrow {}

impl TypeCon for Borrow {}

impl<'a, X : 'a + ?Sized> TypeApp<'a, X> for Borrow
{
  type Applied = &'a X;
}

impl TypeAppGeneric for Borrow
{
  fn with_type_app<'a, X : 'a, R : 'a, Cont : 'a>(cont : Box<Cont>) -> R
  where
    Self : 'a,
    Cont : TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

/// `App<'a, BorrowMutF, X> ~ &'a mut X`
pub enum BorrowMut {}

impl TypeCon for BorrowMut {}

impl<'a, X : 'a + ?Sized> TypeApp<'a, X> for BorrowMut
{
  type Applied = &'a mut X;
}

impl TypeAppGeneric for BorrowMut
{
  fn with_type_app<'a, X : 'a, R : 'a, Cont : 'a>(cont : Box<Cont>) -> R
  where
    Self : 'a,
    Cont : TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

/// `App<BoxF, X> ~ Box<X>`
pub enum BoxF {}
impl_type_app!(BoxF, Box);

/// `App<VecF, X> ~ Vec<X>`
pub enum VecF {}
impl_type_app!(VecF, Vec);

/// `App<OptionF, X> ~ Option<X>`
pub enum OptionF {}
impl_type_app!(OptionF, Option);

/// `App<ResultF<E>, X> ~ Result<E, X>`
pub struct ResultF<E>(PhantomData<E>);
impl_type_app!(ResultF<E>, Result);
