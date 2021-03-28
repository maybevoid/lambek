use std::fmt::Display;

use crate::{
  bi_type_app::*,
  constraint::*,
};

enum DisplayConstraint {}

trait DisplayCont<X: ?Sized, R>
{
  fn on_display(self: Box<Self>) -> R
  where
    X: Display;
}

fn wrap_display_cont<'a, Cont: 'a, X: 'a + ?Sized, R: 'a>(
  cont: Cont
) -> BiApp<'a, DisplayConstraint, X, R>
where
  Cont: DisplayCont<X, R>,
{
  struct Applied<Cont>(Cont);

  impl<'a, Cont: 'a, X: 'a + ?Sized, R: 'a>
    HasBiTypeApp<'a, DisplayConstraint, X, R> for Applied<Cont>
  where
    Cont: DisplayCont<X, R>,
    DisplayConstraint:
      BiTypeApp<'a, X, R, Applied = dyn DisplayCont<X, R> + 'a>,
  {
    fn get_applied(self: Box<Self>) -> Box<dyn DisplayCont<X, R> + 'a>
    {
      Box::new(self.0)
    }

    fn get_applied_borrow(&self) -> &(dyn DisplayCont<X, R> + 'a)
    {
      &self.0
    }

    fn get_applied_borrow_mut(&mut self) -> &mut (dyn DisplayCont<X, R> + 'a)
    {
      &mut self.0
    }
  }

  Box::new(Applied(cont))
}

impl BiTypeCon for DisplayConstraint {}

impl<'a, X: 'a + ?Sized, R: 'a> BiTypeApp<'a, X, R> for DisplayConstraint
{
  type Applied = dyn DisplayCont<X, R> + 'a;
}

impl BiTypeAppGeneric for DisplayConstraint
{
  fn with_type_app<'a, X: 'a, R: 'a, K: 'a>(
    cont: impl BiTypeAppGenericCont<'a, Self, X, R, K>
  ) -> K
  where
    Self: 'a,
  {
    cont.on_type_app()
  }
}

impl<X> HasConstraint<X> for DisplayConstraint
where
  X: Display,
{
  fn with_constraint<'a, R: 'a>(cont1: BiApp<'a, Self, X, R>) -> R
  where
    X: 'a,
  {
    let cont2: Box<dyn DisplayCont<X, R> + 'a> = cont1.get_applied();

    cont2.on_display()
  }
}

fn with_display_constraint<'a, X: 'a, R: 'a>(cont1: impl DisplayCont<X, R>) -> R
where
  DisplayConstraint: HasConstraint<X>,
{
  DisplayConstraint::with_constraint(wrap_display_cont(cont1))
}

#[test]
fn test_display_constraint()
{
  fn use_display<'a, X: 'a>(x: &'a X)
  where
    DisplayConstraint: HasConstraint<X>,
  {
    struct Cont<'a, X: 'a>
    {
      x: &'a X,
    }

    impl<'a, X: 'a> DisplayCont<X, ()> for Cont<'a, X>
    {
      fn on_display(self: Box<Self>)
      where
        X: Display,
      {
        let x = self.x;
        println!("X: {}", x);
      }
    }

    with_display_constraint(Cont { x })
  }

  use_display(&"Hello World".to_string());
}
