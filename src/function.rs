use crate::bi_type_app::*;

pub enum FunctionF {}
pub enum FunctionMutF {}
pub enum FunctionOnceF {}

pub trait IsFnOnce: BiTypeCon
{
  fn apply_once<'a, A: 'a, B: 'a>(
    f: BiApp<'a, Self, A, B>,
    a: A,
  ) -> B;
}

pub trait IsFnMut: IsFnOnce
{
  fn apply_mut<'a, A: 'a, B: 'a>(
    f: &mut BiApp<'a, Self, A, B>,
    a: A,
  ) -> B;
}

pub trait IsFn: IsFnMut
{
  fn apply<'a, A: 'a, B: 'a>(
    f: &BiApp<'a, Self, A, B>,
    a: A,
  ) -> B;
}

impl BiTypeCon for FunctionF {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionF
{
  type Applied = dyn Fn(A) -> B + 'a;
}

impl BiTypeCon for FunctionMutF {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionMutF
{
  type Applied = dyn FnMut(A) -> B + 'a;
}

impl BiTypeCon for FunctionOnceF {}

impl<'a, A: 'a, B: 'a> BiTypeApp<'a, A, B> for FunctionOnceF
{
  type Applied = dyn FnOnce(A) -> B + 'a;
}

impl IsFn for FunctionF
{
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
  F: Fn(A) -> B,
{
  struct Applied<F>(F);

  impl<'a, F: 'a, A: 'a, B: 'a> HasBiTypeApp<'a, FunctionF, A, B> for Applied<F>
  where
    F: Fn(A) -> B,
    FunctionF: BiTypeApp<'a, A, B, Applied = dyn Fn(A) -> B + 'a>,
  {
    fn get_applied_box(self: Box<Self>) -> Box<dyn Fn(A) -> B + 'a>
    {
      Box::new(self.0)
    }

    fn get_applied_borrow(&self) -> &(dyn Fn(A) -> B + 'a)
    {
      &self.0
    }

    fn get_applied_borrow_mut(&mut self) -> &mut (dyn Fn(A) -> B + 'a)
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

pub fn wrap_function_mut<'a, F: 'a, A: 'a, B: 'a>(
  f: F
) -> BiApp<'a, FunctionMutF, A, B>
where
  F: FnMut(A) -> B,
{
  struct Applied<F>(F);

  impl<'a, F: 'a, A: 'a, B: 'a> HasBiTypeApp<'a, FunctionMutF, A, B>
    for Applied<F>
  where
    F: FnMut(A) -> B,
    FunctionMutF: BiTypeApp<'a, A, B, Applied = dyn FnMut(A) -> B + 'a>,
  {
    fn get_applied_box(self: Box<Self>) -> Box<dyn FnMut(A) -> B + 'a>
    {
      Box::new(self.0)
    }

    fn get_applied_borrow(&self) -> &(dyn FnMut(A) -> B + 'a)
    {
      &self.0
    }

    fn get_applied_borrow_mut(&mut self) -> &mut (dyn FnMut(A) -> B + 'a)
    {
      &mut self.0
    }
  }

  Box::new(Applied(f))
}
