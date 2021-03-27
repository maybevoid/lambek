use crate::bi_type_app::*;

pub trait HasConstraint<X: ?Sized>: BiTypeCon
{
  fn with_constraint<'a, R: 'a>(cont: BiApp<'a, Self, X, R>) -> R
  where
    X: 'a,
    Self: 'a;
}
