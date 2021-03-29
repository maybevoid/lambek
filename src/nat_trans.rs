use crate::type_app::*;

pub trait NaturalTransformation<Ref, F, G>
where
  F: TypeAppGeneric,
  G: TypeAppGeneric,
{
  fn lift<'a, 'b, X>(
    ctx: App<'b, Ref, Self>,
    fx: App<'a, F, X>,
  ) -> App<'a, G, X>
  where
    'a: 'b;
}
