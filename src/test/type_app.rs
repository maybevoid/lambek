use std::marker::PhantomData;

use crate::type_app::*;

#[test]
fn test_dyn_with_type_app_cont()
{
  fn on_type_app<'a, F: 'a, X: 'a>(
    x: &u64,
    _applied: &F::Applied,
  ) -> u64
  where
    F: TypeApp<'a, X>,
  {
    x * 2
  }

  fn use_type_app<'a, F: 'a, X: 'a>(applied: App<'a, F, X>) -> u64
  {
    let state: u64 = 42;

    struct Cont<'a, 'b, F: 'a, X: 'a>
    where
      'a: 'b,
    {
      state: &'b u64,
      applied: &'b App<'a, F, X>,
      phantom: PhantomData<&'a (F, X)>,
    }

    impl<'a, 'b, F: 'a, X: 'a> TypeAppCont<'a, F, X, u64> for Cont<'a, 'b, F, X>
    where
      'a: 'b,
    {
      fn on_type_app(self: Box<Self>) -> u64
      where
        F: TypeApp<'a, X>,
      {
        on_type_app::<F, X>(self.state, self.applied.get_applied_borrow())
      }
    }

    let cont = Cont::<F, X> {
      state: &state,
      applied: &applied,
      phantom: PhantomData,
    };

    with_type_app(&applied, cont)
  }

  let res = use_type_app::<Identity, String>(wrap_app("foo".to_string()));

  assert_eq!(res, 84);
}
