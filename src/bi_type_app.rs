use std::marker::PhantomData;

use crate::type_app::*;

// F: BiTypeCon :: Type -> Type -> Type
pub trait BiTypeCon
{
}

pub trait BiTypeApp<'a, X: 'a + ?Sized, Y: 'a + ?Sized>:
  BiTypeCon + 'a
{
  type Applied: 'a + ?Sized;
}

pub trait BiTypeAppGeneric: BiTypeCon + Sized
{
  fn with_type_app<'a, X: 'a, Y: 'a, R: 'a>(
    cont: impl BiTypeAppGenericCont<'a, Self, X, Y, R>
  ) -> R
  where
    Self: 'a;
}

pub trait BiTypeAppGenericCont<'a, F: 'a, X: 'a, Y: 'a, R: 'a>
{
  fn on_type_app(self) -> R
  where
    F: BiTypeApp<'a, X, Y>;
}

pub trait HasBiTypeApp<'a, F: 'a + ?Sized, X: 'a + ?Sized, Y: 'a + ?Sized>
{
  fn get_applied_box(self: Box<Self>) -> Box<F::Applied>
  where
    F: BiTypeApp<'a, X, Y>;

  fn get_applied_borrow(&self) -> &F::Applied
  where
    F: BiTypeApp<'a, X, Y>;

  fn get_applied_borrow_mut(&mut self) -> &mut F::Applied
  where
    F: BiTypeApp<'a, X, Y>;
}

pub type BiApp<'a, F, X, Y> = Box<dyn HasBiTypeApp<'a, F, X, Y> + 'a>;

pub trait ToBiTypeApp<'a, F: 'a + ?Sized, X: 'a + ?Sized, Y: 'a + ?Sized>
{
  fn to_applied(self: Box<Self>) -> BiApp<'a, F, X, Y>;
}

pub fn wrap_bi_app<'a, F: 'a, X: 'a, Y: 'a, FX: 'a>(
  fx: FX
) -> BiApp<'a, F, X, Y>
where
  F: BiTypeApp<'a, X, Y, Applied = FX>,
{
  struct Applied<FX>(FX);

  impl<'a, F: 'a + ?Sized, X: 'a + ?Sized, Y: 'a + ?Sized, FX: 'a>
    HasBiTypeApp<'a, F, X, Y> for Applied<FX>
  where
    F: BiTypeApp<'a, X, Y, Applied = FX>,
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

  Box::new(Applied(fx))
}

/// Partial application of a binary type constructor
/// `F: Type -> Type -> Type` to one type argument `A`,
/// making it a regular type constructor
/// `Partial<F, A>: Type -> Type`.
pub struct Partial<F: ?Sized, A: ?Sized>(PhantomData<F>, PhantomData<A>);

impl<F: ?Sized, A: ?Sized> TypeCon for Partial<F, A> where F: BiTypeCon {}

impl<'a, F: 'a + ?Sized, A: 'a + ?Sized, B: 'a + ?Sized> TypeApp<'a, B>
  for Partial<F, A>
where
  F: BiTypeApp<'a, A, B>,
{
  type Applied = F::Applied;
}
