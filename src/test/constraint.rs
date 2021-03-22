use std::fmt::Display;
use crate::constraint::*;
use crate::send::*;

#[test]
pub fn test_send_constraint () {
  fn use_send_1 < 'a, X > ( x: &'a X ) -> ()
  where
    X: Display,
    X: Send,
  {
    println!("X: {}", x);
  }

  fn use_send < 'a, X > ( x: &'a X )
  where
    X: Display,
    X: HasConstraint < SendConstraint, ContF = SendContF < X > >,
  {
    struct Cont < 'a, X >
    where
      X: Display
    {
      x: &'a X
    }

    impl < 'a, X >
      SendCont < X, () >
      for Cont < 'a, X >
    where
      X: Display
    {
      fn on_send ( self: Box < Self > )
      where
        X: Send
      {
        use_send_1 ( self.x )
      }
    }

    let cont :
      Box < dyn SendCont < X, () > + '_ > =
      Box::new(Cont { x });

    < X as HasConstraint < SendConstraint >
    > :: with_constraint ( Box::new(cont) );
  }

  use_send ( &"Hello World".to_string() );
}
