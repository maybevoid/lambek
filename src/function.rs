use crate::bi_type_app::*;

pub enum FunctionF {}
pub enum FunctionMutF {}
pub enum FunctionOnceF {}

pub trait FnClone<A, B>: Fn(A) -> B
{
  fn clone_fn<'a>(&self) -> Box<dyn FnClone<A, B> + 'a>
  where
    Self: 'a,
    A: 'a,
    B: 'a;

  fn wrap_fn<'a>(self: Box<Self>) -> BiApp<'a, FunctionF, A, B>
  where
    Self: 'a,
    A: 'a,
    B: 'a;
}

pub trait IsFnOnce: BiTypeCon
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>;

  fn apply_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>,
    a: A,
  ) -> B;
}

pub trait IsFnMut: IsFnOnce
{
  fn get_fn_mut<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnMut(A) -> B + 'a>;

  fn apply_mut<'a, A: 'a, B: 'a>(
    f: &mut BiApp<'a, Self, A, B>,
    a: A,
  ) -> B;
}

pub trait IsFn: IsFnMut
{
  fn get_fn<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn Fn(A) -> B + 'a>;

  fn apply<'a, A: 'a, B: 'a>(
    f: &BiApp<'a, Self, A, B>,
    a: A,
  ) -> B;
}

impl BiTypeCon for FunctionF {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionF
{
  type Applied = dyn FnClone<A, B> + 'a;
}

impl BiTypeCon for FunctionMutF {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionMutF
{
  type Applied = Box<dyn FnMut(A) -> B + 'a>;
}

impl BiTypeCon for FunctionOnceF {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionOnceF
{
  type Applied = dyn FnOnce(A) -> B + 'a;
}

impl<A, B, F> FnClone<A, B> for F
where
  F: Fn(A) -> B,
  F: Clone,
{
  fn clone_fn<'a>(&self) -> Box<dyn FnClone<A, B> + 'a>
  where
    Self: 'a,
  {
    Box::new(self.clone())
  }

  fn wrap_fn<'a>(self: Box<Self>) -> BiApp<'a, FunctionF, A, B>
  where
    Self: 'a,
    A: 'a,
    B: 'a,
  {
    wrap_function(*self)
  }
}

impl<'a, A: 'a, B: 'a> Clone for Box<dyn FnClone<A, B> + 'a>
{
  fn clone(&self) -> Self
  {
    self.clone_fn()
  }
}

impl IsFn for FunctionF
{
  fn get_fn<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn Fn(A) -> B + 'a>
  {
    Box::new(f.get_applied_box())
  }

  fn apply<'a, A: 'a, B: 'a>(
    f: &BiApp<'a, Self, A, B>,
    a: A,
  ) -> B
  {
    f.get_applied_borrow()(a)
  }
}

impl IsFnMut for FunctionF
{
  fn get_fn_mut<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnMut(A) -> B + 'a>
  {
    Box::new(f.get_applied_box())
  }

  fn apply_mut<'a, A: 'a, B: 'a>(
    f: &mut BiApp<'a, Self, A, B>,
    a: A,
  ) -> B
  {
    f.get_applied_borrow()(a)
  }
}

impl IsFnOnce for FunctionF
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>
  {
    Box::new(f.get_applied_box())
  }

  fn apply_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>,
    a: A,
  ) -> B
  {
    f.get_applied_box()(a)
  }
}

impl IsFnMut for FunctionMutF
{
  fn get_fn_mut<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnMut(A) -> B + 'a>
  {
    f.get_applied_box()
  }

  fn apply_mut<'a, A: 'a, B: 'a>(
    f: &mut BiApp<'a, Self, A, B>,
    a: A,
  ) -> B
  {
    f.get_applied_borrow_mut()(a)
  }
}

impl IsFnOnce for FunctionMutF
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>
  {
    f.get_applied_box()
  }

  fn apply_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>,
    a: A,
  ) -> B
  {
    f.get_applied_box()(a)
  }
}

impl IsFnOnce for FunctionOnceF
{
  fn get_fn_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>
  ) -> Box<dyn FnOnce(A) -> B + 'a>
  {
    f.get_applied_box()
  }

  fn apply_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>,
    a: A,
  ) -> B
  {
    f.get_applied_box()(a)
  }
}

pub fn wrap_function<'a, F: 'a, A: 'a, B: 'a>(
  f: F
) -> BiApp<'a, FunctionF, A, B>
where
  F: FnClone<A, B>,
{
  struct Applied<F>(F);

  impl<'a, F: 'a, A: 'a, B: 'a> HasBiTypeApp<'a, FunctionF, A, B> for Applied<F>
  where
    F: FnClone<A, B>,
    FunctionF: BiTypeApp<'a, A, B, Applied = dyn FnClone<A, B> + 'a>,
  {
    fn get_applied_box(self: Box<Self>) -> Box<dyn FnClone<A, B> + 'a>
    {
      Box::new(self.0)
    }

    fn get_applied_borrow(&self) -> &(dyn FnClone<A, B> + 'a)
    {
      &self.0
    }

    fn get_applied_borrow_mut(&mut self) -> &mut (dyn FnClone<A, B> + 'a)
    {
      &mut self.0
    }
  }

  Box::new(Applied(f))
}

pub fn wrap_function_once<'a, F: 'a, A: 'a, B: 'a>(
  f: F
) -> BiApp<'a, FunctionOnceF, A, B>
where
  F: FnOnce(A) -> B,
{
  struct Applied<F>(F);

  impl<'a, F: 'a, A: 'a, B: 'a> HasBiTypeApp<'a, FunctionOnceF, A, B>
    for Applied<F>
  where
    F: FnOnce(A) -> B,
    FunctionOnceF: BiTypeApp<'a, A, B, Applied = dyn FnOnce(A) -> B + 'a>,
  {
    fn get_applied_box(self: Box<Self>) -> Box<dyn FnOnce(A) -> B + 'a>
    {
      Box::new(self.0)
    }

    fn get_applied_borrow(&self) -> &(dyn FnOnce(A) -> B + 'a)
    {
      &self.0
    }

    fn get_applied_borrow_mut(&mut self) -> &mut (dyn FnOnce(A) -> B + 'a)
    {
      &mut self.0
    }
  }

  Box::new(Applied(f))
}
