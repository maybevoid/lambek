use crate::type_app::*;

pub trait Functor : TypeAppGeneric
{
  fn fmap < 'a, A, B >
    ( fa: App < 'a, Self, A > )
    -> App < 'a, Self, B >
  where
    Self: 'a
  ;
}

pub trait Applicative : Functor
{
  fn pure < 'a, A: 'a >
    ( a: A )
    -> App < 'a, Self, A >
  ;

  fn apply < 'a, A: 'a, B: 'a, F: 'a >
    ( app: App < 'a, Self, F >,
      fa: App < 'a, Self, A >
    ) ->
      App < 'a, Self, A >
  where
    Self: 'a,
    F: Fn ( A ) -> B
  ;
}

pub trait Monad : Applicative
{
  fn bind < 'a, A: 'a, B: 'a >
    ( ma: App < 'a, Self, A >,
      cont: impl Fn ( A ) -> App < 'a, Self, B >
    ) ->
      App < 'a, Self, B >
  ;
}

pub trait MonadOnce : Applicative
{
  fn bind < 'a, A: 'a, B: 'a >
    ( ma: App < 'a, Self, A >,
      cont: impl FnOnce ( A ) -> App < 'a, Self, B >
    ) ->
      App < 'a, Self, B >
  ;
}
