use crate::{
  function::*,
  functor::*,
  type_app::*,
};

type VecOptionF = Compose<VecF, OptionF>;

#[test]
fn test_fmap_vec_option()
{
  let xs1: Vec<App<OptionF, u64>> =
    vec![wrap_app(Some(42)), wrap_app(None), wrap_app(Some(64))];

  let xs2: App<Compose<VecF, OptionF>, u64> = wrap_app(wrap_app(xs1));

  let mut base = 2;

  let xs3: App<Compose<VecF, OptionF>, String> = VecOptionF::fmap(
    xs2,
    wrap_function_mut(|x| {
      base *= x;
      format!("{}", base)
    }),
  );

  assert_eq!(5376, base);

  let mut xs4: Vec<App<OptionF, String>> = xs3.get_applied().get_applied();

  {
    let x = xs4.pop().unwrap().get_applied();
    assert_eq!(Some("5376".to_string()), x);
  }

  {
    let x = xs4.pop().unwrap().get_applied();
    assert_eq!(None, x);
  }

  {
    let x = xs4.pop().unwrap().get_applied();
    assert_eq!(Some("84".to_string()), x);
  }
}
