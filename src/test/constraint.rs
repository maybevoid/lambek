use std::fmt::Display;

use crate::{
  bi_type_app::*,
  constraint::*,
};

enum DisplayConstraint {}

trait DisplayCont<X : ?Sized, R>
{
  fn on_display(self: Box<Self>) -> R
  where
    X : Display;
}

impl BiTypeCon for DisplayConstraint {}

impl<'a, X : 'a, R : 'a> BiTypeApp<'a, X, R> for DisplayConstraint
{
  type Applied = Box<dyn DisplayCont<X, R> + 'a>;
}

impl BiTypeAppGeneric for DisplayConstraint
{
  fn with_type_app<'a, X : 'a, R : 'a, K : 'a>(
    cont : impl BiTypeAppGenericCont<'a, Self, X, R, K>
  ) -> K
  where
    Self : 'a,
  {
    cont.on_type_app()
  }
}

impl<X> HasConstraint<X> for DisplayConstraint
where
  X : Display,
{
  fn with_constraint<'a, R : 'a>(cont1 : BiApp<'a, Self, X, R>) -> R
  where
    X : 'a,
  {
    let cont2 : Box<dyn DisplayCont<X, R> + 'a> = *cont1.get_applied();

    cont2.on_display()
  }
}

fn with_display_constraint<'a, X : 'a, R : 'a>(
  cont1 : impl DisplayCont<X, R>
) -> R
where
  DisplayConstraint : HasConstraint<X>,
{
  let cont2 : Box<dyn DisplayCont<X, R> + '_> = Box::new(cont1);

  DisplayConstraint::with_constraint(Box::new(cont2))
}

#[test]
fn test_display_constraint()
{
  fn use_display<'a, X : 'a>(x : &'a X)
  where
    DisplayConstraint : HasConstraint<X>,
  {
    struct Cont<'a, X : 'a>
    {
      x : &'a X,
    }

    impl<'a, X : 'a> DisplayCont<X, ()> for Cont<'a, X>
    {
      fn on_display(self: Box<Self>)
      where
        X : Display,
      {
        let x = self.x;
        println!("X: {}", x);
      }
    }

    with_display_constraint(Cont { x })
  }

  use_display(&"Hello World".to_string());
}
