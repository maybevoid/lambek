use crate::{
  nat_trans::*,
  row::*,
  type_app::*,
};

pub trait Product
{
}

pub struct Top;

pub struct Cons<X, Tail>(X, Tail);

impl Product for Top {}

impl<X, Tail> Product for Cons<X, Tail> where Tail: Product {}

impl RowCon for Top {}
impl<X, Tail> RowCon for Cons<X, Tail> {}

impl<'a, F: 'a> RowApp<'a, F> for Top
where
  F: TypeCon,
{
  type Applied = Top;
}

impl<'a, X: 'a, Tail: 'a, F: 'a> RowApp<'a, F> for Cons<X, Tail>
where
  F: TypeCon,
{
  type Applied = Cons<App<'a, F, X>, AppRow<'a, Tail, F>>;
}

impl RowAppGeneric for Top
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

impl<X, Tail> RowAppGeneric for Cons<X, Tail>
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

impl LiftRow for Top
{
  fn lift<'a, F: 'a, G: 'a>(
    _: impl NaturalTransformation<F, G>,
    _: AppRow<'a, Self, F>,
  ) -> AppRow<'a, Self, G>
  where
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    wrap_row(Top)
  }
}

impl<X, Tail> LiftRow for Cons<X, Tail>
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
    let Cons(fx, tail) = *row.get_applied();

    let gx = trans.clone().lift(fx);

    let tail2 = Tail::lift(trans, tail);

    wrap_row(Cons(gx, tail2))
  }
}
