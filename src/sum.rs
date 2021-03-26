use crate::{
  nat_trans::*,
  row::*,
  type_app::*,
};

pub trait Sum
{
}

pub enum Bottom {}

pub enum Union<X, Tail>
{
  Inl(X),
  Inr(Tail),
}

pub use Union::{
  Inl,
  Inr,
};

impl Sum for Bottom {}

impl<X, Tail> Sum for Union<X, Tail> where Tail: Sum {}

impl RowCon for Bottom {}
impl<X, Tail> RowCon for Union<X, Tail> {}

impl<'a, F: 'a> RowApp<'a, F> for Bottom
where
  F: TypeCon,
{
  type Applied = Bottom;
}

impl<'a, X: 'a, Tail: 'a, F: 'a> RowApp<'a, F> for Union<X, Tail>
where
  F: TypeCon,
{
  type Applied = Union<App<'a, F, X>, AppRow<'a, Tail, F>>;
}

impl RowAppGeneric for Bottom
{
  fn with_row_app<'a, F: 'a, R: 'a>(
    cont: impl RowAppGenericCont<'a, Self, F, R>
  ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric,
  {
    cont.on_row_app()
  }
}

impl<X, Tail> RowAppGeneric for Union<X, Tail>
where
  Tail: RowAppGeneric,
{
  fn with_row_app<'a, F: 'a, R: 'a>(
    cont: impl RowAppGenericCont<'a, Self, F, R>
  ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric,
  {
    cont.on_row_app()
  }
}

impl LiftRow for Bottom
{
  fn lift<'a, F: 'a, G: 'a>(
    _: impl NaturalTransformation<F, G>,
    row: AppRow<'a, Self, F>,
  ) -> AppRow<'a, Self, G>
  where
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    match *row.get_applied() {}
  }
}

impl<X, Tail> LiftRow for Union<X, Tail>
where
  Tail: LiftRow,
{
  fn lift<'a, F: 'a, G: 'a>(
    trans: impl NaturalTransformation<F, G> + Clone,
    row: AppRow<'a, Self, F>,
  ) -> AppRow<'a, Self, G>
  where
    Self: 'a,
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    match *row.get_applied() {
      Inl(fx) => wrap_row(Inl(trans.lift(fx))),
      Inr(tail) => wrap_row(Inr(Tail::lift(trans, tail))),
    }
  }
}
