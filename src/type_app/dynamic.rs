use super::base::TypeApp;

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
}

/// Newtype for a boxed value of [HasTypeApp].
pub struct App<'a, F: 'a + ?Sized, X: 'a + ?Sized>(
  pub Box<dyn HasTypeApp<'a, F, X>>,
);

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
}
