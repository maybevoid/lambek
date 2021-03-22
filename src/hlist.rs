use crate::row::*;
use crate::type_app::*;
use crate::nat_trans::*;

pub trait Sum {}
pub trait Product {}

pub struct Top;
pub enum Bottom {}

pub struct Cons
  < X, Tail >
  ( X, Tail );

pub enum Union
  < X, Tail >
{
  Inl ( X ),
  Inr ( Tail ),
}

pub use Union::{ Inl, Inr };

impl Product for Top {}
impl Sum for Bottom {}

impl < X, Tail >
  Product for Cons < X, Tail >
where
  Tail: Product
{ }

impl < X, Tail >
  Sum for Union < X, Tail >
where
  Tail: Sum
{ }

impl RowCon for Top {}
impl RowCon for Bottom {}
impl < X, Tail > RowCon for Cons < X, Tail > { }
impl < X, Tail > RowCon for Union < X, Tail > { }

impl < 'a, F: 'a >
  RowApp < 'a, F >
  for Top
where
  F: TypeCon
{
  type Applied = Top;
}

impl < 'a, F: 'a >
  RowApp < 'a, F >
  for Bottom
where
  F: TypeCon
{
  type Applied = Bottom;
}

impl < 'a, X: 'a, Tail: 'a, F: 'a >
  RowApp < 'a, F >
  for Cons < X, Tail >
where
  F: TypeCon,
{
  type Applied =
    Cons <
      App < 'a, F, X >,
      AppRow < 'a, Tail, F >,
    >;
}

impl < 'a, X: 'a, Tail: 'a, F: 'a >
  RowApp < 'a, F >
  for Union < X, Tail >
where
  F: TypeCon,
{
  type Applied =
    Union <
      App < 'a, F, X >,
      AppRow < 'a, Tail, F >,
    >;
}

impl RowAppGeneric for Top
{
  fn with_row_app < 'a, F: 'a, R: 'a >
    ( cont: impl RowAppGenericCont < 'a, Self, F, R >
    ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric,
  {
    cont.on_row_app()
  }
}

impl RowAppGeneric for Bottom
{
  fn with_row_app < 'a, F: 'a, R: 'a >
    ( cont: impl RowAppGenericCont < 'a, Self, F, R >
    ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric,
  {
    cont.on_row_app()
  }
}

impl < X, Tail >
  RowAppGeneric for Cons < X, Tail >
where
  Tail: RowAppGeneric
{
  fn with_row_app < 'a, F: 'a, R: 'a >
    ( cont: impl RowAppGenericCont < 'a, Self, F, R >
    ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric,
  {
    cont.on_row_app()
  }
}

impl < X, Tail >
  RowAppGeneric for Union < X, Tail >
where
  Tail: RowAppGeneric
{
  fn with_row_app < 'a, F: 'a, R: 'a >
    ( cont: impl RowAppGenericCont < 'a, Self, F, R >
    ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric,
  {
    cont.on_row_app()
  }
}

impl LiftRow for Top {
  fn lift
    < 'a, F: 'a, G: 'a >
    ( _: impl NaturalTransformation < F, G >,
      _: AppRow < 'a, Self, F >
    ) ->
      AppRow < 'a, Self, G >
  where
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    wrap_row ( Top )
  }
}

impl LiftRow for Bottom {
  fn lift
    < 'a, F: 'a, G: 'a >
    ( _: impl NaturalTransformation < F, G >,
      row: AppRow < 'a, Self, F >
    ) ->
      AppRow < 'a, Self, G >
  where
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    match *row.get_applied() { }
  }
}

impl < X, Tail >
  LiftRow for Cons < X, Tail >
where
  Tail: LiftRow
{
  fn lift
    < 'a, F: 'a, G: 'a >
    ( trans: impl NaturalTransformation < F, G >,
      row: AppRow < 'a, Self, F >
    ) ->
      AppRow < 'a, Self, G >
  where
    Self: 'a,
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    let Cons ( fx, tail ) = *row.get_applied();

    let gx = trans.lift ( fx );

    let tail2 = Tail::lift ( trans, tail );

    wrap_row ( Cons ( gx, tail2 ) )
  }
}

impl < X, Tail >
  LiftRow for Union < X, Tail >
where
  Tail: LiftRow
{
  fn lift
    < 'a, F: 'a, G: 'a >
    ( trans: impl NaturalTransformation < F, G >,
      row: AppRow < 'a, Self, F >
    ) ->
      AppRow < 'a, Self, G >
  where
    Self: 'a,
    F: TypeAppGeneric,
    G: TypeAppGeneric,
  {
    match *row.get_applied() {
      Inl ( fx ) => {
        wrap_row ( Inl ( trans.lift ( fx ) ) )
      }
      Inr ( tail ) => {
        wrap_row ( Inr ( Tail::lift ( trans, tail ) ) )
      }
    }
  }
}
