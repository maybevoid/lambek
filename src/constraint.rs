use super::type_app::*;

pub trait HasConstraint < C > : Sized {
  type ContF;

  fn with_constraint < 'a, R: 'a >
    ( cont: App < 'a, Self::ContF, R > )
    -> R
  ;
}
