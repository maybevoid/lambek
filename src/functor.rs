use crate::{
  bi_type_app::*,
  function::*,
  type_app::*,
};

pub trait Functor<Func>: TypeCon
{
  /// `fmap :: forall a b . f a -> (a -> b) -> f b`
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    fa: App<'a, Self, A>,
    mapper: BiApp<'b, Func, A, B>,
  ) -> App<'a, Self, B>
  where
    Self: 'a,
    'a: 'b;
}

pub trait Applicative<Func>: Functor<Func>
{
  fn pure<'a, A: 'a>(a: A) -> App<'a, Self, A>;

  fn apply<'a, 'b, A: 'a, B: 'a, F: 'a>(
    app: App<'a, Self, BiApp<'b, Func, A, B>>,
    fa: App<'a, Self, A>,
  ) -> App<'a, Self, A>
  where
    Self: 'a,
    'a: 'b;
}

pub trait Monad<Func>: Applicative<Func>
{
  fn bind<'a, 'b, A: 'a, B: 'a>(
    ma: App<'a, Self, A>,
    cont: BiApp<'b, Func, A, App<'a, Self, B>>,
  ) -> App<'a, Self, B>
  where
    Self: 'a,
    'a: 'b;
}

impl<F, G> Functor<FunctionOnceF> for Compose<F, G>
where
  F: Functor<FunctionOnceF>,
  G: Functor<FunctionOnceF>,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    xs1: App<'a, Compose<F, G>, A>,
    f1: BiApp<'b, FunctionOnceF, A, B>,
  ) -> App<'a, Compose<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
    'a: 'b,
  {
    let xs2 = xs1.get_applied();
    let g = wrap_function_once(move |ga| G::fmap(ga, f1));
    wrap_app(F::fmap(xs2, g))
  }
}

impl<F, G> Functor<FunctionF> for Compose<F, G>
where
  F: Functor<FunctionF>,
  G: Functor<FunctionF>,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    fga1: App<'a, Compose<F, G>, A>,
    mapper1: BiApp<'b, FunctionF, A, B>,
  ) -> App<'a, Compose<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
    'a: 'b,
  {
    let fga2 = fga1.get_applied();
    let mapper4 = wrap_function(|ga| {
      G::fmap(ga, wrap_function(|x| FunctionF::apply(&mapper1, x)))
    });

    let res = wrap_app(F::fmap(fga2, mapper4));
    res
  }
}

impl<F, G> Functor<FunctionMutF> for Compose<F, G>
where
  F: Functor<FunctionMutF>,
  G: Functor<FunctionMutF>,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    fga1: App<'a, Compose<F, G>, A>,
    mut mapper1: BiApp<'b, FunctionMutF, A, B>,
  ) -> App<'a, Compose<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
    'a: 'b,
  {
    let fga2 = fga1.get_applied();
    let mapper4 = wrap_function_mut(|ga| {
      G::fmap(
        ga,
        wrap_function_mut(|x| FunctionMutF::apply_mut(&mut mapper1, x)),
      )
    });

    let res = wrap_app(F::fmap(fga2, mapper4));
    res
  }
}

impl<Func> Functor<Func> for Identity
where
  Func: IsFnOnce,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    a1: App<'a, Identity, A>,
    f1: BiApp<'b, Func, A, B>,
  ) -> App<'a, Identity, B>
  where
    Self: 'a,
    'a: 'b,
  {
    let a2 = a1.get_applied();
    wrap_app(Func::apply_once(f1, a2))
  }
}

impl<Func, X> Functor<Func> for Const<X>
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    a1: App<'a, Const<X>, A>,
    _: BiApp<'b, Func, A, B>,
  ) -> App<'a, Const<X>, B>
  where
    Self: 'a,
    'a: 'b,
  {
    wrap_app(a1.get_applied())
  }
}

impl<Func> Functor<Func> for OptionF
where
  Func: IsFnOnce,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    a1: App<'a, Self, A>,
    f1: BiApp<'b, Func, A, B>,
  ) -> App<'a, Self, B>
  where
    Self: 'a,
    'a: 'b,
  {
    match a1.get_applied() {
      Some(a2) => wrap_app(Some(Func::apply_once(f1, a2))),
      None => wrap_app(None),
    }
  }
}

impl<Func, E> Functor<Func> for ResultF<E>
where
  Func: IsFnOnce,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    a1: App<'a, Self, A>,
    f1: BiApp<'b, Func, A, B>,
  ) -> App<'a, Self, B>
  where
    Self: 'a,
    'a: 'b,
  {
    match a1.get_applied() {
      Ok(a2) => wrap_app(Ok(Func::apply_once(f1, a2))),
      Err(err) => wrap_app(Err(err)),
    }
  }
}

impl<Func> Functor<Func> for VecF
where
  Func: IsFnMut,
{
  fn fmap<'a, 'b, A: 'a, B: 'a>(
    xs1: App<'a, Self, A>,
    mut f1: BiApp<'b, Func, A, B>,
  ) -> App<'a, Self, B>
  where
    Self: 'a,
    'a: 'b,
  {
    let xs2 = xs1.get_applied();
    let f2 = |x| Func::apply_mut(&mut f1, x);

    let xs3 = xs2.into_iter().map(f2).collect();

    wrap_app(xs3)
  }
}
