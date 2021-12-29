use super::base::{TypeCon, TypeApp};
use super::dynamic::App;

use core::marker::PhantomData;

pub struct Compose<F: ?Sized, G: ?Sized>(PhantomData<F>, PhantomData<G>);

impl<F: ?Sized, G: ?Sized> TypeCon for Compose<F, G> {}

impl<'a, F: 'a + ?Sized, G: 'a + ?Sized, X: 'a + ?Sized, FX: 'a, GX: 'a>
  TypeApp<'a, X> for Compose<F, G>
where
  G: TypeApp<'a, X, Applied = GX>,
  F: TypeApp<'a, GX, Applied = FX>,
{
  type Applied = FX;
}

pub struct ComposeApp<F: ?Sized, G: ?Sized>(PhantomData<F>, PhantomData<G>);

impl<F: ?Sized, G: ?Sized> TypeCon for ComposeApp<F, G> {}

impl<'a, F: 'a + ?Sized, G: 'a + ?Sized, X: 'a + ?Sized> TypeApp<'a, X>
  for ComposeApp<F, G>
{
  type Applied = App<'a, F, App<'a, G, X>>;
}
