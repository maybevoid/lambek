use super::base::{TypeCon, TypeApp};
use crate::refl::{Refl, HasRefl, HasReflUnbounded, refl_symmetric, has_refl};

pub trait TypeAppWitness<'a, F, X: 'a>
{
    type Witness: TypeApp<'a, X> + Refl<Refl=F>;
}

impl <'a, F, X: 'a> TypeAppWitness<'a, F, X> for F
where
    F: TypeApp<'a, X>
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
    trait ReflCont<'a, Cont, X: 'a, R>: Refl {
        type ContRefl: TypeAppCont<'a, Self::Refl, X, R>;

        fn refl_cont(cont: Cont) -> Self::ContRefl;
    }

    impl <'a, Cont, F, X: 'a, R> ReflCont<'a, Cont, X, R> for F
    where
        Cont: TypeAppCont<'a, F, X, R>
    {
        type ContRefl = Cont;

        fn refl_cont(cont: Cont) -> Self::ContRefl {
            cont
        }
    }

    fn refl_cont_inner<'a, Cont, F, X: 'a, R>(
        cont: Cont,
    ) -> impl TypeAppCont<'a, F::Refl, X, R>
    where
        F: ReflCont<'a, Cont, X, R>,
        Cont: TypeAppCont<'a, F, X, R>
    {
        let cont2 = <F as ReflCont<'a, Cont, X, R>>::refl_cont(cont);
        cont2
    }

    fn refl_cont<'a, Cont, F: Refl, X: 'a, R>(
        cont: Cont,
    ) -> impl TypeAppCont<'a, F::Refl, X, R>
    where
        Cont: TypeAppCont<'a, F, X, R>
    {
        refl_cont_inner(cont)
    }

    fn refl_cont_2<'a, Cont, F1, F2, W, X: 'a, R>(
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

    let refl1 = has_refl::<W::Witness, F>();
    let refl2 = refl_symmetric(refl1);
    let cont2 = refl_cont_2(refl2, cont);

    cont2.on_type_app()

    // trait TypeAppWitnessCont<'a, F: 'a, X: 'a>: TypeAppWitness<'a, F, X> {
    //     fn call_cont<R>(
    //         cont: impl TypeAppCont<'a, Self::Witness, X, R>
    //     ) -> R;
    // }

    // impl <'a, W, F: 'a, X: 'a>
    //     TypeAppWitnessCont<'a, F, X>
    //     for W
    // where
    //     W: TypeAppWitness<'a, F, X>
    // {
    //     fn call_cont<R>(
    //         cont: impl TypeAppCont<'a, Self::Witness, X, R>
    //     ) -> R {
    //         cont.on_type_app()
    //     }
    // }

    // trait ReflCont<'a, F: Refl, X: 'a, R> {
    //     type Cont: TypeAppCont<'a, F, X, R>;
    //     type ContRefl: TypeAppCont<'a, F::Refl, X, R>;

    //     fn refl_cont(cont: Self::ContRefl) -> Self::Cont;
    // }

    // impl <'a, Cont, F, X: 'a, R> ReflCont<'a, F, X, R> for Cont
    // where
    //     Cont: TypeAppCont<'a, F, X, R>
    // {
    //     type Cont = Cont;
    //     type ContRefl = Cont;

    //     fn refl_cont(cont: Self::ContRefl) -> Self::Cont {
    //         cont
    //     }
    // }

    // fn refl_cont<'a, Cont, F: Refl, X: 'a, R>(
    //     cont: Cont::ContRefl
    // ) -> Cont::Cont
    // where
    //     Cont: ReflCont<'a, F, X, R>
    // {
    //     Cont::refl_cont(cont)
    // }


    // fn refl_cont_2<'a, Cont, F: Refl, X: 'a, R>(
    //     cont: Cont,
    // ) -> impl TypeAppCont<'a, F, X, R>
    // where
    //     Cont: TypeAppCont<'a, F::Refl, X, R>,
    // {
    //     Cont::refl_cont(cont)
    // }

    // trait ReflCont<'a, Cont, X: 'a, R>: Refl
    // where
    //     Cont: TypeAppCont<'a, Self::Refl, X, R>
    // {
    //     type ContRefl: TypeAppCont<'a, Self, X, R>;

    //     fn refl_cont(
    //         cont: Cont
    //     ) -> Self::ContRefl;
    // }

    // impl <'a, F, Cont, X: 'a, R> ReflCont<'a, Cont, X, R> for F
    // where
    //     F: Refl<Refl=F>,
    //     Cont: TypeAppCont<'a, F::Refl, X, R>
    // {
    //     type ContRefl = Cont;

    //     fn refl_cont(
    //         cont: Cont
    //     ) -> Self::ContRefl {
    //         cont
    //     }
    // }

    // fn with_type_app_witness_inner<'a, Cont, F1: 'a, F2: 'a, X: 'a, R>(
    //     cont1: Cont,
    // ) -> R
    // where
    //     F1: Refl,
    //     F1::Refl: Sized,
    //     F1: TypeApp<'a, X>,
    //     Cont: TypeAppCont<'a, F1::Refl, X, R>,
    // {
    //     let cont2 = refl_cont::<'a, Cont, F1, X, R>(cont1);
    //     cont2.on_type_app()
    // }

    // let cont2 = <W::Witness as ReflCont<'a, Cont, X, R>>::refl_cont(cont);


    // W::call_cont(W::Witness::refl_cont(cont))
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
