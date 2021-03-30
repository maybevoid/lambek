use crate::{
  nat_trans::*,
  type_app::*,
};

// Row: RowCon :: (Type -> Type) -> Type
pub trait RowCon
{
}

pub trait RowApp<'a, F: 'a + ?Sized>: RowCon
where
  F: TypeCon,
{
  type Applied: 'a;
}

pub trait RowAppGeneric: RowCon + Sized
{
  fn with_row_app<'a, F: 'a, R: 'a>(
    cont: impl RowAppGenericCont<'a, Self, F, R>
  ) -> R
  where
    Self: 'a,
    F: TypeAppGeneric;
}

pub trait RowAppGenericCont<'a, Row: 'a, F: 'a, R: 'a>
{
  fn on_row_app(self) -> R
  where
    F: TypeCon,
    Row: RowApp<'a, F>;
}

pub trait HasRowApp<'a, Row: 'a + ?Sized, F: 'a + ?Sized + TypeCon>
{
  fn get_applied(self: Box<Self>) -> Box<Row::Applied>
  where
    Row: RowApp<'a, F>;

  fn get_applied_borrow<'b>(&'b self) -> &'b Row::Applied
  where
    Row: RowApp<'a, F>;

  fn get_applied_borrow_mut<'b>(&'b mut self) -> &'b mut Row::Applied
  where
    Row: RowApp<'a, F>;
}

pub type AppRow<'a, Row, F> = Box<dyn HasRowApp<'a, Row, F> + 'a>;

pub fn wrap_row<'a, Row: 'a, F: 'a>(row: Row::Applied) -> AppRow<'a, Row, F>
where
  F: TypeCon,
  Row: RowApp<'a, F>,
{
  Box::new(row)
}

impl<'a, Row: 'a, F: 'a, RF: 'a> HasRowApp<'a, Row, F> for RF
where
  F: TypeCon,
  Row: RowApp<'a, F, Applied = RF>,
{
  fn get_applied(self: Box<Self>) -> Box<RF>
  {
    self
  }

  fn get_applied_borrow(&self) -> &RF
  {
    self
  }

  fn get_applied_borrow_mut(&mut self) -> &mut RF
  {
    self
  }
}

pub trait LiftRow<Ref>: RowCon
where
  Ref: TypeCon,
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
    G: TypeAppGeneric;
}
