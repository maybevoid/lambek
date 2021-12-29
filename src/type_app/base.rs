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
