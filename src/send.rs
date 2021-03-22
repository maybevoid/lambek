use std::marker::PhantomData;

use super::type_app::*;
use super::constraint::*;

pub enum SendConstraint {}

pub struct SendContF < X > ( PhantomData < X > );

pub trait SendCont < X, R > {
  fn on_send ( self: Box < Self > ) -> R
  where
    X: Send
  ;
}

impl < X >
  TypeCon
  for SendContF < X >
{ }

impl < 'a, X: 'a, R: 'a >
  TypeApp < 'a, R >
  for SendContF < X >
{
  type Applied = Box < dyn SendCont < X, R > + 'a >;
}

impl < A >
  TypeAppGeneric for SendContF < A >
{
  fn with_type_app < 'a, X: 'a, R: 'a >
    ( cont: impl TypeAppGenericCont < 'a, Self, X, R > )
    -> R
  where
    Self: 'a
  {
    cont.on_type_app()
  }
}

impl < X: 'static >
  HasConstraint < SendConstraint >
  for X
where
  X: Send
{
  type ContF = SendContF < X >;

  fn with_constraint < 'a, R: 'a >
    ( cont1: App < 'a, Self::ContF, R > )
    -> R
  {
    let cont2: Box < dyn SendCont < X, R > + 'a > =
      *cont1.get_applied();

    cont2.on_send()
  }
}

pub fn with_send_constraint
  < 'a, X, R >
  ( cont1: impl SendCont < X, R > )
  -> R
where
  X: HasConstraint < SendConstraint, ContF = SendContF < X > >,
{
  let cont2: Box < dyn SendCont < X, R > + '_ > =
    Box::new ( cont1 );

  < X as HasConstraint < SendConstraint >
  > :: with_constraint ( Box::new(cont2) )
}

// #[macro_export]
// macro_rules! wrap_send_func {
//   (
//     fn $name:ident
//       < $( $lifetime_param:lifetime $(,)? )* $(,)?
//         $( $type_param:ident $(,)? )*
//       >
//     where
//       $( $type_spec:ty : $constraint:tt $(,)? )*
//     deriving ( $constraint_type:ty )
//       $derive_spec:ty : $derive_constraint:tt $(,)?
//     $body:expr
//   ) => {

//   }
// }
