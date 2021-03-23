use std::fmt::Display;
use std::marker::PhantomData;

use crate::constraint::*;
use crate::bi_type_app::*;

enum DisplayConstraint {}

trait DisplayCont < X, R > {
  fn on_display ( self: Box < Self > ) -> R
  where
    X: Display
  ;
}

impl BiTypeCon for DisplayConstraint { }

impl < 'a, X: 'a, R: 'a >
  BiTypeApp < 'a, X, R >
  for DisplayConstraint
{
  type Applied = Box < dyn DisplayCont < X, R > + 'a >;
}

impl BiTypeAppGeneric for DisplayConstraint
{
  fn with_type_app < 'a, X: 'a, R: 'a, K: 'a >
    ( cont: impl BiTypeAppGenericCont < 'a, Self, X, R, K > )
    -> K
  where
    Self: 'a
  {
    cont.on_type_app()
  }
}

impl < X >
  HasConstraint < X >
  for DisplayConstraint
where
  X: Display
{
  fn with_constraint < 'a, R: 'a >
    ( cont1: BiApp < 'a, Self, X, R > )
    -> R
  where
    X: 'a
  {
    let cont2: Box < dyn DisplayCont < X, R > + 'a > =
      *cont1.get_applied();

    cont2.on_display()
  }
}

fn with_display_constraint
  < 'a, X: 'a, R: 'a >
  ( cont1: impl DisplayCont < X, R > )
  -> R
where
  DisplayConstraint:
    HasConstraint <
      X,
    >,
{
  let cont2: Box < dyn DisplayCont < X, R > + '_ > =
    Box::new ( cont1 );

  DisplayConstraint::with_constraint
    ( Box::new ( cont2 ) )
}

#[test]
fn test_display_constraint () {
  struct UseDisplay < 'a, X > ( PhantomData < &'a X > );

  impl < 'a, X >
    DisplayCont < X, fn (&'a X) >
    for UseDisplay < 'a, X >
  {
    fn on_display ( self: Box < Self > )
      -> fn (&'a X)
    where
      X: Display
    {
      Self::use_display_1
    }
  }

  impl < 'a, X >
    UseDisplay < 'a, X >
  {
    fn use_display_1 ( x: &'a X )
    where
      X: Display,
    {
      println!("X: {}", x);
    }

    fn use_display ( x: &'a X )
    where
      DisplayConstraint:
        HasConstraint <
          X,
        >,
    {
      with_display_constraint( UseDisplay (PhantomData) )
      ( x )
    }
  }

  UseDisplay::use_display ( &"Hello World".to_string() );
}
