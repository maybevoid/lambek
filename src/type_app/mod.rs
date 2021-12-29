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

pub mod base;
pub mod compose;
pub mod identity;
pub mod generic;
pub mod dynamic;

pub use base::*;
pub use compose::*;
pub use identity::*;
pub use generic::*;
pub use dynamic::*;

use core::marker::PhantomData;

impl<F> TypeAppGeneric for F
where
  F: TypeAppGenericUnsized,
{
  fn with_type_app<'a, X: 'a, R: 'a, Cont: 'a>(cont: Cont) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    TypeAppGenericUnsized::with_type_app(cont)
  }
}

pub trait CloneApp: TypeCon
{
  fn clone_app<'a, X: 'a>(fx: &App<'a, Self, X>) -> App<'a, Self, X>;
}

/// Wraps a type `FX` into [App] in the presence of the [TypeApp]
/// constraint, allowing subsequent use of [App] to not depend
/// on [TypeApp].
pub fn wrap_app<'a, F: 'a, X: 'a, FX: 'a>(fx: FX) -> App<'a, F, X>
where
  F: TypeApp<'a, X, Applied = FX>,
{
  struct Applied<X>(X);

  impl<'a, F: 'a, X: 'a, FX: 'a> HasTypeApp<'a, F, X> for Applied<FX>
  where
    F: TypeApp<'a, X, Applied = FX>,
  {
    fn get_applied_box(self: Box<Self>) -> Box<FX>
    {
      Box::new(self.0)
    }

    fn get_applied_borrow(&self) -> &FX
    {
      &self.0
    }

    fn get_applied_borrow_mut(&mut self) -> &mut FX
    {
      &mut self.0
    }
  }

  App(Box::new(Applied(fx)))
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
        cont : Cont
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

/// `App<Const<A>, X> ~ A`
///
/// A type `X` applied to `Const<A>` simply has the type argument
/// discarded. So the type application result is always `A`.
///
/// Unlike in Haskell, the `Applied` result can just be an `A`,
/// instead of getting wrapped around a newtype.
pub struct Const<A: ?Sized>(PhantomData<A>);

impl<A: ?Sized> TypeCon for Const<A> {}

impl<'a, A: 'a + ?Sized, X: 'a + ?Sized> TypeApp<'a, X> for Const<A>
{
  type Applied = A;
}

pub struct AppF<F: ?Sized>(PhantomData<F>);

impl<F: ?Sized> TypeCon for AppF<F> {}

impl<'a, X: 'a + ?Sized, F: 'a + ?Sized> TypeApp<'a, X> for AppF<F>
{
  type Applied = App<'a, F, X>;
}

/// `App<VecF, X> ~ Vec<X>`
pub enum VecF {}
impl_type_app!(VecF, Vec);

/// `App<OptionF, X> ~ Option<X>`
pub enum OptionF {}
impl_type_app!(OptionF, Option);

/// `App<ResultF<E>, X> ~ Result<E, X>`
pub struct ResultF<E>(PhantomData<E>);

impl<E> TypeCon for ResultF<E> {}

impl<'a, E: 'a, X: 'a> TypeApp<'a, X> for ResultF<E>
{
  type Applied = Result<X, E>;
}
