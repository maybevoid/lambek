// F: BiTypeCon :: Type -> Type -> Type
pub trait BiTypeCon
{
}

pub trait BiTypeApp<'a, X : 'a + ?Sized, Y : 'a + ?Sized>: BiTypeCon
{
  type Applied: 'a + ?Sized;
}

pub trait BiTypeAppGeneric: BiTypeCon + Sized
{
  fn with_type_app<'a, X : 'a, Y : 'a, R : 'a>(
    cont : impl BiTypeAppGenericCont<'a, Self, X, Y, R>
  ) -> R
  where
    Self : 'a;
}

pub trait BiTypeAppGenericCont<'a, F : 'a, X : 'a, Y : 'a, R : 'a>
{
  fn on_type_app(self) -> R
  where
    F : BiTypeApp<'a, X, Y>;
}

pub trait HasBiTypeApp<'a, F : 'a + ?Sized, X : 'a + ?Sized, Y : 'a + ?Sized>
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F : BiTypeApp<'a, X, Y>;

  fn get_applied_borrow<'b>(&'b self) -> &'b F::Applied
  where
    F : BiTypeApp<'a, X, Y>;

  fn get_applied_borrow_mut<'b>(&'b mut self) -> &'b mut F::Applied
  where
    F : BiTypeApp<'a, X, Y>;
}

pub type BiApp<'a, F, X, Y> = Box<dyn HasBiTypeApp<'a, F, X, Y> + 'a>;

pub fn wrap_app<'a, F : 'a, X : 'a, Y : 'a, FX : 'a>(
  fx : FX
) -> BiApp<'a, F, X, Y>
where
  F : BiTypeApp<'a, X, Y, Applied = FX>,
{
  Box::new(fx)
}

impl<'a, F : 'a, X : 'a, Y : 'a, FX : 'a> HasBiTypeApp<'a, F, X, Y> for FX
where
  F : BiTypeApp<'a, X, Y, Applied = FX>,
{
  fn get_applied(self: Box<Self>) -> Box<FX>
  {
    self
  }

  fn get_applied_borrow<'b>(&'b self) -> &'b FX
  {
    self
  }

  fn get_applied_borrow_mut<'b>(&'b mut self) -> &'b mut FX
  {
    self
  }
}
