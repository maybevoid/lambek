use crate::{
  nat_trans::*,
  row::*,
  type_app::*,
};

pub trait Sum {}

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

impl<Ref> LiftRow<Ref> for Bottom
where
  Ref: TypeCon,
{
  fn lift<'a, 'b, F: 'a, G: 'a, Trans>(
    _: App<'b, Ref, Trans>,
    row: AppRow<'a, Self, F>,
  ) -> AppRow<'a, Self, G>
  where
    'a: 'b,
    Self: 'a,
    Trans: NaturalTransformation<Ref, F, G>,
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    match *row.get_applied() {}
  }
}

impl<Ref, X, Tail> LiftRow<Ref> for Union<X, Tail>
where
  Ref: TypeCon,
  Tail: LiftRow<Ref>,
{
  fn lift<'a, 'b, F: 'a, G: 'a, Trans>(
    trans: App<'b, Ref, Trans>,
    row: AppRow<'a, Self, F>,
  ) -> AppRow<'a, Self, G>
  where
    'a: 'b,
    Self: 'a,
    Trans: NaturalTransformation<Ref, F, G>,
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    match *row.get_applied() {
      Inl(fx) => wrap_row(Inl(Trans::lift(trans, fx))),
      Inr(tail) => wrap_row(Inr(Tail::lift(trans, tail))),
    }
  }
}
