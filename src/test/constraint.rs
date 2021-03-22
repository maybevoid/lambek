use std::fmt::Display;
use std::marker::PhantomData;

use crate::constraint::*;
use crate::send::*;

#[test]
pub fn test_send_constraint () {
  struct UseSend < 'a, X > ( PhantomData < &'a X > );

  impl < 'a, X >
    SendCont < X, fn (&'a X) >
    for UseSend < 'a, X >
  where
    X: Display
  {
    fn on_send ( self: Box < Self > )
      -> fn (&'a X)
    where
      X: Send
    {
      Self::use_send_1
    }
  }

  impl < 'a, X >
    UseSend < 'a, X >
  where
    X: Display,
  {
    fn use_send_1 ( x: &'a X )
    where
      X: Send,
    {
      println!("X: {}", x);
    }

    fn use_send ( x: &'a X )
    where
      X: HasConstraint < SendConstraint, ContF = SendContF < X > >,
    {
      with_send_constraint( UseSend (PhantomData) )
      ( x )
    }
  }

  UseSend::use_send ( &"Hello World".to_string() );
}
