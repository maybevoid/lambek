use std::marker::PhantomData;

pub trait TypeCon { }

pub trait TypeApp < 'a, X: 'a + ?Sized > : TypeCon {
  type Applied : 'a + ?Sized;
}

pub trait TypeAppGeneric : TypeCon + Sized {
  fn with_type_app < 'a, X: 'a, R: 'a >
    ( cont: impl TypeAppGenericCont < 'a, Self, X, R > )
    -> R
  where
    Self: 'a
  ;
}

pub trait TypeAppGenericCont
  < 'a, F: 'a, X: 'a, R: 'a >
{
  fn on_type_app ( self ) -> R
  where
    F: TypeApp < 'a, X >
  ;
}

pub trait HasTypeApp < 'a, F: 'a + ?Sized, X: 'a + ?Sized >
{
  fn get_applied ( self: Box < Self > )
    -> Box < F::Applied >
  where
    F: TypeApp < 'a, X >
  ;

  fn get_applied_borrow < 'b >
    ( &'b self )
    -> &'b F::Applied
  where
    F: TypeApp < 'a, X >
  ;

  fn get_applied_borrow_mut < 'b >
    ( &'b mut self )
    -> &'b mut F::Applied
  where
    F: TypeApp < 'a, X >
  ;
}

pub type App < 'a, F, X > = Box < dyn HasTypeApp < 'a, F, X > + 'a >;

pub fn wrap_app < 'a, F: 'a, X: 'a, FX: 'a >
  ( fx: FX )
  -> App < 'a, F, X >
where
  F: TypeApp < 'a, X, Applied=FX >
{
  Box::new ( fx )
}

impl < 'a, F: 'a, X: 'a, FX: 'a >
  HasTypeApp < 'a, F, X >
  for FX
where
  F: TypeApp < 'a, X, Applied = FX >
{
  fn get_applied ( self: Box < Self > )
    -> Box < FX >
  {
    self
  }

  fn get_applied_borrow < 'b >
    ( &'b self )
    -> &'b FX
  {
    self
  }

  fn get_applied_borrow_mut < 'b >
    ( &'b mut self )
    -> &'b mut FX
  {
    self
  }
}

pub enum Identity {}

impl TypeCon for Identity { }

impl TypeAppGeneric for Identity {
  fn with_type_app < 'a, X: 'a, R: 'a >
    ( cont: impl TypeAppGenericCont < 'a, Self, X, R > )
    -> R
  where
    Self: 'a
  {
    cont.on_type_app()
  }
}

impl < 'a, X: 'a + ?Sized > TypeApp < 'a, X > for Identity {
  type Applied = X;
}

pub struct Const < A: ?Sized > ( PhantomData < A > );

impl < A: ?Sized > TypeCon for Const < A > {}

impl < 'a, A: 'a + ?Sized, X: 'a + ?Sized >
  TypeApp < 'a, X > for Const < A >
{
  type Applied = A;
}

pub enum Borrow {}

impl TypeCon for Borrow {}

impl < 'a, X: 'a + ?Sized > TypeApp < 'a, X > for Borrow {
  type Applied = &'a X;
}

impl TypeAppGeneric for Borrow {
  fn with_type_app < 'a, X: 'a, R: 'a >
    ( cont: impl TypeAppGenericCont < 'a, Self, X, R > )
    -> R
  where
    Self: 'a
  {
    cont.on_type_app()
  }
}

pub enum BorrowMut {}

impl TypeCon for BorrowMut {}

impl < 'a, X: 'a + ?Sized > TypeApp < 'a, X > for BorrowMut {
  type Applied = &'a mut X;
}

impl TypeAppGeneric for BorrowMut {
  fn with_type_app < 'a, X: 'a, R: 'a >
    ( cont: impl TypeAppGenericCont < 'a, Self, X, R > )
    -> R
  where
    Self: 'a
  {
    cont.on_type_app()
  }
}
