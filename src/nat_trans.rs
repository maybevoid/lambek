use crate::type_app::*;

pub trait NaturalTransformation<F, G>
where
  F: TypeAppGeneric,
  G: TypeAppGeneric,
{
  fn lift<X>(
    self,
    fx: App<'_, F, X>,
  ) -> App<'_, G, X>;
}
