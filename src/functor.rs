use crate::{
  bi_type_app::*,
  function::*,
  type_app::*,
};

pub trait Functor<Fn>: TypeCon
{
  /// `fmap :: forall a b . f a -> (a -> b) -> f b`
  fn fmap<'a, A: 'a, B: 'a>(
    fa: App<'a, Self, A>,
    mapper: BiApp<'a, Fn, A, B>,
  ) -> App<'a, Self, B>
  where
    Self: 'a;
}

pub trait Applicative<Fn>: Functor<Fn>
{
  fn pure<'a, A: 'a>(a: A) -> App<'a, Self, A>;

  fn apply<'a, A: 'a, B: 'a, F: 'a>(
    app: App<'a, Self, BiApp<'a, Fn, A, B>>,
    fa: App<'a, Self, A>,
  ) -> App<'a, Self, A>
  where
    Self: 'a;
}

pub trait Monad<Fn>: Applicative<Fn>
{
  fn bind<'a, A: 'a, B: 'a>(
    ma: App<'a, Self, A>,
    cont: BiApp<Fn, A, App<'a, Self, B>>,
  ) -> App<'a, Self, B>;
}

impl<F, G> Functor<FunctionOnceF> for ComposeF<F, G>
where
  F: Functor<FunctionOnceF>,
  G: Functor<FunctionOnceF>,
{
  fn fmap<'a, A: 'a, B: 'a>(
    xs1: App<'a, ComposeF<F, G>, A>,
    f1: BiApp<'a, FunctionOnceF, A, B>,
  ) -> App<'a, ComposeF<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
  {
    let xs2 = xs1.get_applied();
    let g = wrap_function_once(move |ga| G::fmap(ga, f1));
    wrap_app(F::fmap(xs2, g))
  }
}

impl<F, G> Functor<FunctionF> for ComposeF<F, G>
where
  F: Functor<FunctionF>,
  G: Functor<FunctionF>,
{
  fn fmap<'a, A: 'a, B: 'a>(
    fga1: App<'a, ComposeF<F, G>, A>,
    mapper1: BiApp<'a, FunctionF, A, B>,
  ) -> App<'a, ComposeF<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
  {
    let fga2 = fga1.get_applied();
    let mapper2 = mapper1.get_applied_box();
    let mapper4 =
      wrap_function(move |ga| G::fmap(ga, mapper2.clone().wrap_fn()));

    let res = wrap_app(F::fmap(fga2, mapper4));
    res
  }
}
