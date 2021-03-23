use crate::type_app::*;

pub trait NaturalTransformation<F, G>
where
  F : TypeAppGeneric,
  G : TypeAppGeneric,
{
  fn lift<'a, X>(
    &self,
    fx : App<'a, F, X>,
  ) -> App<'a, G, X>;
}
