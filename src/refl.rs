/*!
   Implementation for the type quality constraint, a.k.a. reflexivity.

   This module introduces the `Refl` trait with an associated type `Refl`,
   which is implemented for all types with the `Refl::Refl` being the
   same type that implements `Refl`.

   ```
   trait Refl {
       type Refl;
   }

   impl <T> Refl for T {
       type Refl = T;
   }
   ```

   This means that if we have `T1: Refl<Refl=T2>`, that means that `T1` and
   `T2` are in fact the same type and can be used interchangeably.

   The `Refl` trait by itself do not have much use, as Rust currently have
   amnesia on the concrete type of an associated type. For example, the
   following code would fail to compile, even though conceptually it should
   succeed:

   ```compile_fail
   use lambek::refl::Refl;

   fn reflect_self<T1, T2>(x: T1) -> T2
   where
       T1: Refl<Refl=T2>,
   {
       x
   }
   ```

   We have already know that `T1` implements `Refl` and `T1::Refl` must be
   also `T1`, so `T2` must be equal to `T1` and so we should be able to return
   `x` as `T2` instead of `T1`. But the Rust compiler currently have amnesia
   and forgot the concrete type of `T1::Refl`. So it cannot conclude that
   `T1` and `T2` are equal.

   Fortunately, there is in fact a way for us to help Rust recover from
   amnesia, and remember back the concrete type for `T1::Refl`. We do this
   by defining a trait [`ReflSelf`] that extends [`Refl`], with a generic
   implementation of how to reflect back itself:

   ```rust
   # use lambek::refl::Refl;
   trait ReflSelf: Refl {
       fn refl_self(self) -> Self::Refl;
   }

   impl <T> ReflSelf for T {
       fn refl_self(self) -> Self::Refl {
           self
       }
   }

   fn reflect_self<T1, T2>(x: T1) -> T2
   where
       T1: Refl<Refl=T2>,
   {
       T1::refl_self(x)
   }
   ```

   The trait [`ReflSelf`] implements [`Refl`] and also provide a method
   [`refl_self`](ReflSelf::refl_self) that accepts a `self` and returns
   `Self::Refl`. Since we know `Self::Refl` is in fact the same as `Self`,
   the implementation is trivial with us simply returning `self`.

   Notice that in the generic implementation of [`ReflSelf`], we state
   that the trait is implemented for _all_ types. This is allowed because
   in the trait implementation scope, Rust do look up the trait
   implementation for `Refl`, and find out that `Refl` is implemented
   for all types. In other words, at this stage, Rust have not have amnesia
   and still remember that the concrete type for `Refl::Refl` is in fact
   `Self`. Using that fact, we easily implement
   [`refl_self`](ReflSelf::refl_self) to return `self` without Rust
   complaining.

   Using [`ReflSelf`], we help put a sticky note in Rust's mind, and we
   can use that to remind Rust again what the type of `Refl::Refl` is
   when we need it. We now try to implement `reflect_self` again,
   which now calls [`ReflSelf::refl_self`] to convert `T1` to `T1::Refl`.
   But since we also specify the constraint `T1: Refl<Refl=T2>`, that
   means Rust actually thinks that the type `T1::Refl` returned by
   [`ReflSelf::refl_self`] must also be `T2`, and therefore the code
   compiles successfully.

   Using the same technique, we can also construct new traits that extend
   `Refl` to perform any reflexivity conversion that we need. For example,
   let's say we have a `Vec<T1>` we know that `T1: Refl<Refl=T2>`.
   Following that we should be able to convert our `Vec<T1>` into a
   `Vec<T2>`. So we can define a `ReflVec` trait like follows:

   ```rust
   use lambek::refl::Refl;

   pub trait ReflVec: Refl {
       fn refl_vec(left: Vec<Self>) -> Vec<Self::Refl>;
   }

   impl <X> ReflVec for X {
       fn refl_vec(left: Vec<Self>) -> Vec<Self::Refl> {
           left
       }
   }

   pub fn reflect_vec<T1, T2>(xs: Vec<T1>) -> Vec<T2>
   where
       T1: Refl<Refl=T2>,
   {
       T1::refl_vec(xs)
   }
   ```

   We can also use the technique to cast anything, including references and
   boxed values, so that the conversion can be done at zero cost. `lambek`
   also provides the [`ReflApp`] trait, so that users can easily reflect
   any higher kinded types using the proxy types that implement [`TypeApp`].
*/

use crate::{
  reference::Borrow,
  type_app::{
    App,
    AppF,
    Applied,
    Compose,
    OptionF,
    TypeApp,
  },
};

pub trait Refl: Sized
{
  type Refl: Sized;
}

impl<T> Refl for T
{
  type Refl = T;
}

pub trait HasRefl<T1, T2>
{
  type T1: Refl<Refl = Self::T2>;
  type T2: Refl<Refl = Self::T1>;
}

impl<A, T> HasRefl<T, T> for A
{
  type T1 = T;
  type T2 = T;
}

pub fn has_refl<T1, T2>(
) -> impl HasRefl<T1, T2, T1 = T1, T2 = T2> + HasRefl<T2, T1, T1 = T2, T2 = T1>
where
  T1: Refl<Refl = T2>,
{
  trait ReflWitness: Refl
  {
    type Witness: HasRefl<Self, Self::Refl, T1 = Self, T2 = Self::Refl>
      + HasRefl<Self::Refl, Self, T1 = Self::Refl, T2 = Self>;

    fn witness() -> Self::Witness;
  }

  impl<T> ReflWitness for T
  {
    type Witness = ();

    fn witness() -> Self::Witness {}
  }

  fn has_refl_inner<T1, T2>(
  ) -> impl HasRefl<T1, T2, T1 = T1, T2 = T2> + HasRefl<T2, T1, T1 = T2, T2 = T1>
  where
    T1: Refl<Refl = T2>,
    T1: ReflWitness,
  {
    T1::witness()
  }

  has_refl_inner::<T1, T2>()
}

pub fn refl_symmetric<W, T1, T2>() -> impl HasRefl<T2, T1, T1 = T2, T2 = T1>
where
  W: HasRefl<T1, T2, T1 = T1, T2 = T2>,
{
  fn refl_symmetric_inner<W, T1, T2>(
  ) -> impl HasRefl<W::T2, W::T1, T1 = W::T2, T2 = W::T1>
  where
    W: HasRefl<T1, T2>,
  {
    has_refl::<W::T1, W::T2>()
  }

  refl_symmetric_inner::<W, T1, T2>()
}

pub fn refl_transitive<T1, T2, T3>() -> impl HasRefl<T1, T3, T1 = T1, T2 = T3>
where
  T1: Refl<Refl = T2>,
  T2: Refl<Refl = T3>,
{
  trait ReflTransitive: Refl
  {
    type Witness: HasRefl<
      Self,
      <Self::Refl as Refl>::Refl,
      T1 = Self,
      T2 = <Self::Refl as Refl>::Refl,
    >;

    fn witness() -> Self::Witness;
  }

  impl<T> ReflTransitive for T
  {
    type Witness = ();

    fn witness() -> Self::Witness {}
  }

  fn refl_transitive_inner<T: ReflTransitive>(
  ) -> impl HasRefl<T, <T::Refl as Refl>::Refl, T1 = T, T2 = <T::Refl as Refl>::Refl>
  {
    T::witness()
  }

  refl_transitive_inner::<T1>()
}

pub fn vec_congruence<T1, T2>() -> impl HasRefl<Vec<T1>, Vec<T2>>
where
  T1: Refl<Refl = T2>,
{
  trait VecCongruence: Refl
  {
    type Witness: HasRefl<Vec<Self>, Vec<Self::Refl>>;
    fn witness() -> Self::Witness;
  }

  impl<T> VecCongruence for T
  {
    type Witness = ();
    fn witness() -> Self::Witness {}
  }

  fn vec_congruence_inner<T>() -> impl HasRefl<Vec<T>, Vec<T::Refl>>
  where
    T: VecCongruence,
  {
    T::witness()
  }

  vec_congruence_inner::<T1>()
}

pub fn app_congruence<'a, F: 'a, T1: 'a, T2: 'a>(
) -> impl HasRefl<<F as TypeApp<'a, T1>>::Applied, <F as TypeApp<'a, T2>>::Applied>
where
  T1: Refl<Refl = T2>,
  F: TypeApp<'a, T1>,
  F: TypeApp<'a, T2>,
  <F as TypeApp<'a, T1>>::Applied: Sized,
  <F as TypeApp<'a, T2>>::Applied: Sized,
{
  trait AppCongruence<'a, F>: Refl + 'a
  where
    F: TypeApp<'a, Self>,
    F: TypeApp<'a, Self::Refl>,
    <F as TypeApp<'a, Self>>::Applied: Sized,
    <F as TypeApp<'a, Self::Refl>>::Applied: Sized,
  {
    type Witness: HasRefl<
      <F as TypeApp<'a, Self>>::Applied,
      <F as TypeApp<'a, Self::Refl>>::Applied,
    >;
    fn witness() -> Self::Witness;
  }

  impl<'a, F, T: 'a> AppCongruence<'a, F> for T
  where
    F: TypeApp<'a, Self>,
    F: TypeApp<'a, Self::Refl>,
    <F as TypeApp<'a, Self>>::Applied: Sized,
    <F as TypeApp<'a, Self::Refl>>::Applied: Sized,
  {
    type Witness = ();
    fn witness() -> Self::Witness {}
  }

  fn app_congruence_inner<'a, F, T>() -> impl HasRefl<
    <F as TypeApp<'a, T>>::Applied,
    <F as TypeApp<'a, T::Refl>>::Applied,
  >
  where
    T: AppCongruence<'a, F>,
    F: TypeApp<'a, T>,
    F: TypeApp<'a, T::Refl>,
    <F as TypeApp<'a, T>>::Applied: Sized,
    <F as TypeApp<'a, T::Refl>>::Applied: Sized,
  {
    T::witness()
  }

  app_congruence_inner::<'a, F, T1>()
}

pub fn option_congruence<'a, T1: 'a, T2: 'a>(
) -> impl HasRefl<Option<T1>, Option<T2>> + 'a
where
  T1: Refl<Refl = T2>,
{
  app_congruence::<'a, OptionF, T1, T2>()
}

pub fn option_ref_congruence<'a, T1: 'a, T2: 'a>(
) -> impl HasRefl<&'a Option<T1>, &'a Option<T2>>
where
  T1: Refl<Refl = T2>,
{
  app_congruence::<'a, Compose<Borrow, OptionF>, T1, T2>()
}

/**
   Extend `Refl` to provide reflection on base values.
*/
pub trait ReflSelf: Refl
{
  /**
     Reflect a `self` type
  */
  fn refl_self(self) -> Self::Refl;
  fn refl_self_right(x: Self::Refl) -> Self;

  /**
     Reflect a `&self`
  */
  fn refl_self_ref(&self) -> &Self::Refl;

  /**
     Reflect `&mut self`
  */
  fn refl_self_mut(&mut self) -> &mut Self::Refl;

  /**
     Reflect `Box<Self>`
  */
  fn refl_self_box(self: Box<Self>) -> Box<Self::Refl>;

  fn refl_right_ref(right: &Self::Refl) -> &Self;
}

impl<T> ReflSelf for T
{
  fn refl_self(self) -> Self::Refl
  {
    self
  }

  fn refl_self_right(x: Self::Refl) -> Self
  {
    x
  }

  fn refl_self_ref(&self) -> &Self::Refl
  {
    self
  }

  fn refl_self_mut(&mut self) -> &mut Self::Refl
  {
    self
  }

  fn refl_self_box(self: Box<Self>) -> Box<Self::Refl>
  {
    self
  }

  fn refl_right_ref(right: &Self::Refl) -> &Self
  {
    right
  }
}

/**
   Reflect higher kinded types when there is a type equality for the
   argument types.

   We make use of the [`type_app`](crate::type_app) module which define high
   kinded types to help us reflect type equalities within other types.

   For example, since [`Applied<OptionF, T1>`] is the same as `Option<T1`>,
   we can use
   [`OptionF`](crate::type_app::OptionF)`::`[`refl_app`](ReflApp::refl_app)
   to reflect a `Option<T1>` into `Option<T2>` if `T1: Refl<Refl=T2>`:


   ```rust
   use lambek::refl::{Refl, ReflApp};
   use lambek::type_app::OptionF;

   pub fn reflect_option<T1, T2>(xs: Option<T1>) -> Option<T2>
   where
       T1: Refl<Refl = T2>,
   {
       T1::refl_app::<OptionF>(xs)
   }
   ```

   For reflecting more complex types such as `&Option<T1>`, we can also use
   [`OptionF`](crate::type_app::OptionF) together with the HKT combinator
   [`Compose`](crate::type_app::Compose) and the
   [`Borrow`](crate::reference::Borrow) type. Together, the type
   [`Applied<Compose<Borrow, OptionF>, T1>`] is the same as `&Option<T1>`.
   Hence we can use [`ReflApp`] to reflect that with
   [`Compose<Borrow, OptionF>`](crate::type_app::Compose) being the
   HKT reflector:

   ```rust
   use lambek::refl::{Refl, ReflApp};
   use lambek::type_app::{Compose, OptionF};
   use lambek::reference::Borrow;

   pub fn reflect_option_borrow<T1, T2>(xs: &Option<T1>) -> &Option<T2>
   where
       T1: Refl<Refl = T2>,
   {
       T1::refl_app::<Compose<Borrow, OptionF>>(xs)
   }

   ```
*/
pub trait ReflApp: Refl
{
  fn refl_app<'a, F>(
    applied: <F as TypeApp<'a, Self>>::Applied
  ) -> <F as TypeApp<'a, Self::Refl>>::Applied
  where
    Self: 'a,
    F: TypeApp<'a, Self>,
    F: TypeApp<'a, Self::Refl>,
    <F as TypeApp<'a, Self>>::Applied: Sized,
    <F as TypeApp<'a, Self::Refl>>::Applied: Sized;
}

impl<T> ReflApp for T
{
  fn refl_app<'a, F>(
    applied: Applied<'a, F, Self>
  ) -> Applied<'a, F, Self::Refl>
  where
    Self: 'a,
    F: TypeApp<'a, Self>,
    F: TypeApp<'a, Self::Refl>,
    Applied<'a, F, Self>: Sized,
    Applied<'a, F, Self::Refl>: Sized,
  {
    applied
  }
}

pub fn reflect_value<T1, T2>(x: T1) -> T2
where
  T1: Refl<Refl = T2>,
{
  T1::refl_self(x)
}

pub fn reflect_value_right<T1, T2>(x: T2) -> T1
where
  T1: Refl<Refl = T2>,
{
  T1::refl_self_right(x)
}

pub fn reflect_value_ref<T1, T2>(x: &T1) -> &T2
where
  T1: Refl<Refl = T2>,
{
  T1::refl_self_ref(x)
}

pub fn reflect_right_ref<T1, T2>(x: &T2) -> &T1
where
  T1: Refl<Refl = T2>,
{
  T1::refl_right_ref(x)
}

pub fn reflect_value_mut<T1, T2>(x: &mut T1) -> &mut T2
where
  T1: Refl<Refl = T2>,
{
  T1::refl_self_mut(x)
}

pub fn reflect_boxed<T1, T2>(x: Box<T1>) -> Box<T2>
where
  T1: Refl<Refl = T2>,
{
  T1::refl_self_box(x)
}

pub fn reflect_applied<F, T1, T2>(xs: App<F, T1>) -> App<F, T2>
where
  T1: Refl<Refl = T2>,
{
  T1::refl_app::<AppF<F>>(xs)
}
