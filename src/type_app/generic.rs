use super::base::{
  TypeApp,
  TypeCon,
};
use crate::refl::{
  has_refl,
  refl_symmetric,
  HasRefl,
  HasReflUnbounded,
  Refl,
};

pub trait TypeAppWitness<'a, F, X: 'a>
{
  type Witness: TypeApp<'a, X> + Refl<Refl = F>;
}

impl<'a, F, X: 'a> TypeAppWitness<'a, F, X> for F
where
  F: TypeApp<'a, X>,
{
  type Witness = Self;
}

pub fn with_type_app_witness<'a, W, Cont, F: 'a, X: 'a, R>(
  _: W,
  cont: Cont,
) -> R
where
  W: TypeAppWitness<'a, F, X>,
  Cont: TypeAppCont<'a, F, X, R>,
{
  trait ReflCont<'a, Cont, X: 'a, R>: Refl
  {
    type ContRefl: TypeAppCont<'a, Self::Refl, X, R>;

    fn refl_cont(cont: Cont) -> Self::ContRefl;
  }

  impl<'a, Cont, F, X: 'a, R> ReflCont<'a, Cont, X, R> for F
  where
    Cont: TypeAppCont<'a, F, X, R>,
  {
    type ContRefl = Cont;

    fn refl_cont(cont: Cont) -> Self::ContRefl
    {
      cont
    }
  }

  fn refl_cont_inner<'a, Cont, F, X: 'a, R>(
    cont: Cont
  ) -> impl TypeAppCont<'a, F::Refl, X, R>
  where
    F: ReflCont<'a, Cont, X, R>,
    Cont: TypeAppCont<'a, F, X, R>,
  {
    F::refl_cont(cont)
  }

  fn refl_cont<'a, Cont, F: Refl, X: 'a, R>(
    cont: Cont
  ) -> impl TypeAppCont<'a, F::Refl, X, R>
  where
    Cont: TypeAppCont<'a, F, X, R>,
  {
    refl_cont_inner(cont)
  }

  fn inner_1<'a, Cont, F1, F2, W, X: 'a, R>(
    _: W,
    cont: Cont,
  ) -> impl TypeAppCont<'a, W::Right, X, R>
  where
    Cont: TypeAppCont<'a, W::Left, X, R>,
    W: HasReflUnbounded<F1, F2>,
    W::Left: Sized,
    W::Right: Sized,
  {
    refl_cont::<'a, Cont, W::Left, X, R>(cont)
  }

  fn inner_2<'a, W, F1, F2, X: 'a, R>(
    w: W,
    cont: impl TypeAppCont<'a, F1, X, R>,
  ) -> R
  where
    W: HasRefl<F1, F2>,
    F2: TypeApp<'a, X>,
  {
    let cont2 = inner_1(w, cont);
    cont2.on_type_app()
  }

  fn inner_3<'a, W, F1, F2, X: 'a, R>(
    w1: W,
    cont: impl TypeAppCont<'a, F2, X, R>,
  ) -> R
  where
    W: HasRefl<F1, F2>,
    F1: TypeApp<'a, X>,
  {
    let w2 = refl_symmetric(w1);
    inner_2(w2, cont)
  }

  fn inner_4<'a, Cont, F, X: 'a, R>(cont: Cont) -> R
  where
    F: Refl + TypeApp<'a, X>,
    F::Refl: Sized,
    Cont: TypeAppCont<'a, F::Refl, X, R>,
  {
    inner_3(has_refl::<F, _>(), cont)
  }

  inner_4::<'a, Cont, W::Witness, X, R>(cont)
}

pub trait TypeAppGeneric: TypeCon + Sized
{
  fn with_type_app<'a, X: 'a, R: 'a, Cont: 'a>(cont: Cont) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>;
}

pub trait TypeAppGenericUnsized: TypeCon
{
  fn with_type_app<'a, X: 'a + ?Sized, R: 'a, Cont: 'a>(cont: Cont) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>;
}

pub trait TypeAppCont<'a, F: ?Sized, X: 'a + ?Sized, R>
{
  fn on_type_app(self) -> R
  where
    F: TypeApp<'a, X>;
}
