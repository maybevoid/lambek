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
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
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

pub fn wrap_app<'a, F: 'a, X: 'a, Y: 'a, FX: 'a>(fx: FX) -> BiApp<'a, F, X, Y>
where
  F: BiTypeApp<'a, X, Y, Applied = FX>,
{
  Box::new(fx)
}

impl<'a, F: 'a + ?Sized, X: 'a + ?Sized, Y: 'a + ?Sized, FX: 'a + ?Sized>
  HasBiTypeApp<'a, F, X, Y> for FX
where
  F: BiTypeApp<'a, X, Y, Applied = FX>,
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

pub trait IsFn: IsFnMut
{
  fn get_fn<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn Fn(A) -> B + 'a>;
}

pub trait IsFnMut: IsFnOnce
{
  fn get_fn_mut<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnMut(A) -> B + 'a>;
}

pub trait IsFnOnce: BiTypeCon
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>;
}

pub enum Function {}
pub enum FunctionMut {}
pub enum FunctionOnce {}

impl BiTypeCon for Function {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for Function
{
  type Applied = Box<dyn FnClone<'a, A, B>>;
}

impl BiTypeCon for FunctionMut {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionMut
{
  type Applied = Box<dyn FnMut(A) -> B + 'a>;
}

impl BiTypeCon for FunctionOnce {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionOnce
{
  type Applied = Box<dyn FnOnce(A) -> B + 'a>;
}

pub trait FnClone<'a, A: 'a, B: 'a>: Fn(A) -> B + 'a
{
  fn clone_fn(&self) -> Box<dyn FnClone<'a, A, B>>;
}

impl<'a, A: 'a, B: 'a, F: 'a> FnClone<'a, A, B> for F
where
  F: Fn(A) -> B,
  F: Clone,
{
  fn clone_fn(&self) -> Box<dyn FnClone<'a, A, B>>
  {
    Box::new(self.clone())
  }
}

impl<'a, A: 'a, B: 'a> Clone for Box<dyn FnClone<'a, A, B>>
{
  fn clone(&self) -> Self
  {
    self.clone_fn()
  }
}

impl IsFn for Function
{
  fn get_fn<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn Fn(A) -> B + 'a>
  {
    f.get_applied()
  }
}

impl IsFnMut for Function
{
  fn get_fn_mut<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnMut(A) -> B + 'a>
  {
    f.get_applied()
  }
}

impl IsFnOnce for Function
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>
  {
    f.get_applied()
  }
}

impl IsFnMut for FunctionMut
{
  fn get_fn_mut<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnMut(A) -> B + 'a>
  {
    f.get_applied()
  }
}

impl IsFnOnce for FunctionMut
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>
  {
    f.get_applied()
  }
}

impl IsFnOnce for FunctionOnce
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>
  {
    f.get_applied()
  }
}
