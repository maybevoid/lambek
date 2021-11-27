use core::fmt::Display;

pub trait HasDisplay<X>
{
  type T: Display;
}

impl<A, X: Display> HasDisplay<X> for A
{
  type T = X;
}

pub trait Show<X>: HasDisplay<X>
{
  fn show(x: &Self::T) -> String;
}

impl<A: HasDisplay<X>, X> Show<X> for A
{
  fn show(x: &Self::T) -> String
  {
    format!("{}", x)
  }
}

pub fn show<C: HasDisplay<X, T = X>, X>(
  _: &C,
  x: &X,
) -> String
{
  C::show(x)
}

pub fn display_witness<X: Display>() -> impl HasDisplay<X, T = X> {}

pub fn display_witness2<X>(
  witness: impl HasDisplay<X, T = X>
) -> impl HasDisplay<X, T = X>
{
  witness
}

fn main()
{
  let res = show(&display_witness(), &"hello world");
  println!("{}", res);
}
