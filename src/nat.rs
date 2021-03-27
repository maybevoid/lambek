use std::marker::PhantomData;

pub trait Nat
{
  #[allow(non_upper_case_globals)]
  const Value: Self;

  fn new() -> Self;
}

#[derive(Copy, Clone)]
pub struct Z;

#[derive(Copy, Clone)]
pub struct S<N>(pub PhantomData<N>);

impl Nat for Z
{
  #[allow(non_upper_case_globals)]
  const Value: Z = Z;

  fn new() -> Z
  {
    Z
  }
}

impl<N> Nat for S<N>
where
  N: Nat,
{
  #[allow(non_upper_case_globals)]
  const Value: S<N> = S(PhantomData);

  fn new() -> S<N>
  {
    S(PhantomData)
  }
}

pub fn succ<N>(_: N) -> S<N>
{
  S(PhantomData)
}
