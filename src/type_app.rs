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

use std::marker::PhantomData;

/// A proxy type `F: TypeCon` to mark itself as having the kind
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
pub trait TypeCon {}

/// A type `F: TypeApp<X>` have the associated type `Applied` as the
/// result of applying a type `F` of kind `Type -> Type` to `X`.
///
/// More specifically, `TypeApp` is also parameterized by a lifetime
/// `'a` to support application of types with limited lifetime.
/// Unlike other functional languages, the higher kinded type
/// application `F X` have to consider the case that both `F` and `X`
/// may have different lifetimes. To deal with that, we require that
/// a type `X` can only be applied to a type `F` if they both share
/// a common lifetime `'a`. The result of the type application must
/// also have the lifetime `'a`.
///
/// In practice, we typically define `F` to have `'static` lifetime,
/// i.e. they do not contain references. On the other hand the type
/// argument `X` _may_ contain references. For example, the result
/// of `VecF: TypeApp<'a, &'aX>` would be `Vec<&'a X>`. A typical
/// implementation of `TypeApp` would something like follows:
///
/// ```
/// # use lambek::type_app::*;
/// enum FooF {}
/// struct Foo<X>(X);
/// impl TypeCon for FooF {}
/// impl<'a, X: 'a> TypeApp<'a, X> for FooF
/// {
///   type Applied = Foo<X>;
/// }
/// ```
///
/// A type constructor `F` may also choose to implement `TypeApp`
/// for _unsized_ type arguments X, e.g. `dyn` trait objects.
/// For example, we could define a type `BarF` to make the result
/// of applying a type `X` into `dyn Bar<X>`:
///
/// ```
/// # use lambek::type_app::*;
/// enum BarF {}
/// trait Bar<X>
/// {
///   fn bar(
///     &self,
///     x: X,
///   );
/// }
/// impl TypeCon for BarF {}
/// impl<'a, X: 'a> TypeApp<'a, X> for BarF
/// {
///   type Applied = dyn Bar<X> + 'a;
/// }
/// ```
pub trait TypeApp<'a, X: 'a + ?Sized>: TypeCon + 'a
{
  type Applied: 'a + ?Sized;
}

pub type Applied<'a, F, X> = <F as TypeApp<'a, X>>::Applied;

pub trait TypeAppGeneric: TypeCon + Sized
{
  fn with_type_app<'a, X: 'a, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>;
}

pub trait TypeAppGenericUnsized: TypeCon
{
  fn with_type_app<'a, X: 'a + ?Sized, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>;
}

pub trait TypeAppCont<'a, F: 'a + ?Sized, X: 'a + ?Sized, R: 'a>
{
  fn on_type_app(self: Box<Self>) -> R
  where
    F: TypeApp<'a, X>;
}

impl<F> TypeAppGeneric for F
where
  F: TypeAppGenericUnsized,
{
  fn with_type_app<'a, X: 'a, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    TypeAppGenericUnsized::with_type_app(cont)
  }
}

/// Encapsulates an applied type into a trait object to prevent
/// `TypeApp` constraints from propagating to type signatures.
///
/// A weakness of using [TypeApp] directly is that the trait bounds
/// for every application is propagated to the type signatures
/// that use them. Consider the Haskell `fmap` function of type
/// `forall a b . f a -> (a -> b) -> f b`. If we naively use
/// `TypeApp` to encode `fmap` in Rust, it would become something
/// like:
///
/// ```
/// # use lambek::type_app::*;
/// trait Functor
/// {
///   fn fmap<'a, A, B>(
///     fa: <Self as TypeApp<'a, A>>::Applied,
///     map: impl Fn(A) -> B,
///   ) -> <Self as TypeApp<'a, B>>::Applied
///   where
///     Self: TypeApp<'a, A>,
///     Self: TypeApp<'a, B>;
/// }
/// ```
///
/// To use the above version of `fmap`, we would have to satisfy
/// the two constraints `TypeApp<'a, A>` and `TypeApp<'a, B>`,
/// even if we know a type `F` implements `TypeApp` for all
/// types. This constraint can easily get out of hand especially
/// if we use [TypeApp] within some higher abstractions such as
/// [RowApp](crate::row::RowApp).
///
/// Notice that in most cases, functions like `fmap` treat the
/// applied types as opaque, so they don't really need to know
/// the concrete `Applied` type. We can therefore encapsulates
/// the applied type into a trait object, and then convert it
/// back to the concrete type only when we need it.
///
/// The `HasTypeApp` trait is implemented for all `Applied`
/// associated type arise from any `F: TypeApp<'a, X>`.
/// We wrap an `Applied` type into a
/// `Box<dyn HasTypeApp<'a, F, X>>` to discharge the
/// `TypeApp` constraint. When we need the concrete type
/// again, we then call [get_applied_box](HasTypeApp::get_applied_box)
/// which again requires the `TypeApp` trait bound to be present.
///
/// Using `HasTypeApp`, the trait `Functor` can instead be
/// defined as:
///
/// ```
/// # use lambek::type_app::*;
/// trait Functor
/// {
///   fn fmap<'a, A, B>(
///     fa: Box<dyn HasTypeApp<'a, Self, A>>,
///     f: impl Fn(A) -> B,
///   ) -> Box<dyn HasTypeApp<'a, Self, A>>;
/// }
/// ```
///
/// Notice that the `TypeApp` constraint is now gone, and code
/// that use `fmap` no longer need to know whether a type `F`
/// really implements `TypeApp` for all `X`. We can also use
/// the type alias [App] so that we can write `App<'a, F, X>`
/// instead of `Box<dyn HasTypeApp<'a, F, X>>`.
///
/// A downside of using `HasTypeApp` is that applied types have
/// to be wrapped as a boxed trait object, which involves heap
/// allocation. However the overhead can be minimal if the
/// boxed values are reference types such as `Box<&FX>`.
/// Take this consideration into account when you define a
/// type constructor.
pub trait HasTypeApp<'a, F: 'a + ?Sized, X: 'a + ?Sized>: 'a
{
  /// Get an applied type `FX` out of a
  /// `Box<dyn HasTypeApp<'a, F, X>>` with the trait bound
  /// `F: TypeApp<'a, X>` present.
  fn get_applied_box(self: Box<Self>) -> Box<F::Applied>
  where
    F: TypeApp<'a, X>;

  /// Get an reference to the applied type, `&FX`, out of a
  /// `&dyn HasTypeApp<'a, F, X>` with the trait bound
  /// `F: TypeApp<'a, X>` present.
  fn get_applied_borrow(&self) -> &F::Applied
  where
    F: TypeApp<'a, X>;

  /// Get a mutable reference to the applied type, `&mut FX`,
  /// out of a `&mut dyn HasTypeApp<'a, F, X>` with the trait bound
  /// `F: TypeApp<'a, X>` present.
  fn get_applied_borrow_mut(&mut self) -> &mut F::Applied
  where
    F: TypeApp<'a, X>;

  /// If we have a Rust value of type `&dyn HasTypeApp<'a, F, X>`,
  /// we want to know for sure that `F` implements `TypeApp<'a, X>`.
  /// We can use CPS to ask for a witness of such implementation
  /// by calling `with_type_app` with a continuation implementing
  /// [TypeAppCont]. The continuation is then called with the
  /// trait bound `F: TypeApp<'a, X>` present.
  fn with_type_app<'b>(
    &self,
    cont: Box<dyn TypeAppCont<'a, F, X, ()> + 'b>,
  ) where
    'a: 'b;
}

/// Newtype for a boxed value of [HasTypeApp].
pub struct App<'a, F: 'a + ?Sized, X: 'a + ?Sized>(
  pub Box<dyn HasTypeApp<'a, F, X>>,
);

pub trait CloneApp: TypeCon
{
  fn clone_app<'a, X: 'a>(fx: &App<'a, Self, X>) -> App<'a, Self, X>;
}

pub fn with_type_app<'a, 'b, F: 'a + ?Sized, X: 'a + ?Sized, R: 'a>(
  applied: &'b App<'a, F, X>,
  cont: impl TypeAppCont<'a, F, X, R>,
) -> R
where
  'a: 'b,
{
  struct WrapCont<'b, Cont, R>
  {
    cont: Box<Cont>,
    result: &'b mut Option<R>,
    phantom: PhantomData<R>,
  }

  impl<'a, 'b, F: 'a + ?Sized, X: 'a + ?Sized, R: 'a, Cont>
    TypeAppCont<'a, F, X, ()> for WrapCont<'b, Cont, R>
  where
    Cont: TypeAppCont<'a, F, X, R>,
  {
    fn on_type_app(self: Box<Self>)
    where
      F: TypeApp<'a, X>,
    {
      let res = self.cont.on_type_app();
      *self.result = Some(res);
    }
  }

  let mut result: Option<R> = None;
  let wrap_cont = WrapCont {
    cont: Box::new(cont),
    result: &mut result,
    phantom: PhantomData,
  };

  applied.with_type_app(Box::new(wrap_cont));

  result.unwrap()
}

impl<'a, F: 'a + ?Sized, X: 'a + ?Sized> App<'a, F, X>
{
  pub fn get_applied(self) -> F::Applied
  where
    F: TypeApp<'a, X>,
    F::Applied: Sized,
  {
    *self.0.get_applied_box()
  }
}

impl<'a, F: 'a + ?Sized, X: 'a + ?Sized> HasTypeApp<'a, F, X> for App<'a, F, X>
{
  fn get_applied_box(self: Box<Self>) -> Box<F::Applied>
  where
    F: TypeApp<'a, X>,
  {
    self.0.get_applied_box()
  }

  /// Get an reference to the applied type, `&FX`, out of a
  /// `&dyn HasTypeApp<'a, F, X>` with the trait bound
  /// `F: TypeApp<'a, X>` present.
  fn get_applied_borrow(&self) -> &F::Applied
  where
    F: TypeApp<'a, X>,
  {
    self.0.get_applied_borrow()
  }

  fn get_applied_borrow_mut(&mut self) -> &mut F::Applied
  where
    F: TypeApp<'a, X>,
  {
    self.0.get_applied_borrow_mut()
  }

  fn with_type_app<'b>(
    &self,
    cont: Box<dyn TypeAppCont<'a, F, X, ()> + 'b>,
  ) where
    'a: 'b,
  {
    self.0.with_type_app(cont)
  }
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

    fn with_type_app<'b>(
      &self,
      cont: Box<dyn TypeAppCont<'a, F, X, ()> + 'b>,
    ) where
      'a: 'b,
    {
      cont.on_type_app()
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

pub struct Compose<F: ?Sized, G: ?Sized>(PhantomData<F>, PhantomData<G>);

impl<F: ?Sized, G: ?Sized> TypeCon for Compose<F, G> {}

impl<'a, F: 'a + ?Sized, G: 'a + ?Sized, X: 'a + ?Sized, FX: 'a, GX: 'a>
  TypeApp<'a, X> for Compose<F, G>
where
  G: TypeApp<'a, X, Applied = GX>,
  F: TypeApp<'a, GX, Applied = FX>,
{
  type Applied = FX;
}

pub struct ComposeApp<F: ?Sized, G: ?Sized>(PhantomData<F>, PhantomData<G>);

impl<F: ?Sized, G: ?Sized> TypeCon for ComposeApp<F, G> {}

impl<'a, F: 'a + ?Sized, G: 'a + ?Sized, X: 'a + ?Sized> TypeApp<'a, X>
  for ComposeApp<F, G>
{
  type Applied = App<'a, F, App<'a, G, X>>;
}

/// `App<Identity, X> ~ X`
///
/// A type `X` applied to `Identity` always give us back `X` itself.
///
/// Unlike in Haskell, the `Applied` result can just be an `X`,
/// instead of getting wrapped around a newtype.
pub enum Identity {}

impl TypeCon for Identity {}

impl<'a, X: 'a + ?Sized> TypeApp<'a, X> for Identity
{
  type Applied = X;
}

impl TypeAppGeneric for Identity
{
  fn with_type_app<'a, X: 'a, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
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
