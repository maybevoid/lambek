use crate::{
  bi_type_app::*,
  type_app::*,
};

pub trait Functor<Fn>: TypeCon
{
  /// `fmap :: forall a b . f a -> (a -> b) -> f b`
  fn fmap<'a, A: 'a, B: 'a>(
    fa: App<'a, Self, A>,
    f: BiApp<'a, Fn, A, B>,
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

impl<F, G> Functor<FunctionOnce> for ComposeF<F, G>
where
  F: Functor<FunctionOnce>,
  G: Functor<FunctionOnce>,
{
  fn fmap<'a, A: 'a, B: 'a>(
    xs1: App<'a, ComposeF<F, G>, A>,
    f1: BiApp<'a, FunctionOnce, A, B>,
  ) -> App<'a, ComposeF<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
  {
    let Compose(xs2) = *xs1.get_applied();

    #[allow(clippy::type_complexity)]
    let g: Box<dyn FnOnce(App<'a, G, A>) -> App<'a, G, B> + 'a> =
      Box::new(move |ga| G::fmap(ga, f1));

    Box::new(Compose(F::fmap(xs2, Box::new(g))))
  }
}

impl<F, G> Functor<Function> for ComposeF<F, G>
where
  F: Functor<Function>,
  G: Functor<Function>,
{
  fn fmap<'a, A: 'a, B: 'a>(
    xs1: App<'a, ComposeF<F, G>, A>,
    f1: BiApp<'a, Function, A, B>,
  ) -> App<'a, ComposeF<F, G>, B>
  where
    Self: 'a,
    F: 'a,
    G: 'a,
  {
    let Compose(xs2) = *xs1.get_applied();
    let f2 = *f1.get_applied();

    let g: Box<dyn FnClone<'a, App<'a, G, A>, App<'a, G, B>>> =
      Box::new(move |ga| G::fmap(ga, Box::new(f2.clone_fn())));

    Box::new(Compose(F::fmap(xs2, Box::new(g))))
  }
}
