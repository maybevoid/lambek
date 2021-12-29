use super::{
  base::{
    TypeApp,
    TypeCon,
  },
  generic::{
    TypeAppCont,
    TypeAppGeneric,
  },
};

/// `App<Identity, X> ~ X`
///
/// A type `X` applied to `Identity` always give us back `X` itself.
///
/// Unlike in Haskell, the `Applied` result can just be an `X`,
/// instead of getting wrapped around a newtype.
pub enum Identity {}

impl TypeCon for Identity {}

impl<'a, X: 'a + ?Sized> TypeApp<'a, X> for Identity
{
  type Applied = X;
}

impl TypeAppGeneric for Identity
{
  fn with_type_app<'a, X: 'a, R: 'a, Cont: 'a>(cont: Cont) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}
