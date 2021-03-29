use crate::type_app::*;

/// Allows functions to be polymorphic over both mutable and immutable
/// references.
///
/// With HKT, we can turn reference types into type constructors
/// and define generic functions that work on multiple reference
/// types. The `IsRef` trait is implemented for both [Borrow]
/// and [BorrowMut], which after type application becomes
/// `&X` and `&mut X`, respectively.
pub trait IsRef: TypeCon
{
  fn get_ref<'a, 'b, X: 'a + ?Sized>(x: &'b App<'a, Self, X>) -> &'b X
  where
    'a: 'b;
}

pub trait IsMutRef: IsRef
{
  fn get_mut_ref<'a, 'b, X: 'a + ?Sized>(
    x: &'b mut App<'a, Self, X>
  ) -> &'b mut X
  where
    'a: 'b;
}

pub trait IsOwn: IsMutRef
{
  fn get_own<'a, X: 'a>(x: App<'a, Self, X>) -> X;
}

pub trait IsBox: IsOwn
{
  fn get_box<'a, X: 'a + ?Sized>(x: App<'a, Self, X>) -> Box<X>;
}

/// `App<'a, Borrow, X> ~ &'a X`
///
/// The result of applying `Borrow` to `X` becomes `&X`.
/// Note that Borrow also implements [TypeApp] on unsized
/// types. So we can also apply Borrow to `dyn Trait` objects.
/// i.e. `App<'a, Borrow, dyn Trait> ~ &'a dyn Trait`.
pub enum Borrow {}

/// `App<'a, BorrowMut, X> ~ &'a mut X`
pub enum BorrowMut {}

/// `App<BoxF, X> ~ Box<X>`
pub enum BoxF {}

/// `App<Own, X> ~ X`
pub enum Own {}

impl TypeCon for Borrow {}

impl<'a, X: 'a + ?Sized> TypeApp<'a, X> for Borrow
{
  type Applied = &'a X;
}

impl TypeAppGenericUnsized for Borrow
{
  fn with_type_app<'a, X: 'a + ?Sized, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

impl IsRef for Borrow
{
  fn get_ref<'a, 'b, X: 'a + ?Sized>(x: &'b App<'a, Self, X>) -> &'b X
  where
    'a: 'b,
  {
    x.get_applied_borrow()
  }
}

impl CloneApp for Borrow
{
  fn clone_app<'a, X: 'a>(fx: &App<'a, Self, X>) -> App<'a, Self, X>
  {
    wrap_app(fx.get_applied_borrow().clone())
  }
}

impl IsRef for BorrowMut
{
  fn get_ref<'a, 'b, X: 'a + ?Sized>(x: &'b App<'a, Self, X>) -> &'b X
  where
    'a: 'b,
  {
    x.get_applied_borrow()
  }
}

impl IsMutRef for BorrowMut
{
  fn get_mut_ref<'a, 'b, X: 'a + ?Sized>(
    x: &'b mut App<'a, Self, X>
  ) -> &'b mut X
  where
    'a: 'b,
  {
    x.get_applied_borrow_mut()
  }
}

impl TypeCon for BorrowMut {}

impl<'a, X: 'a + ?Sized> TypeApp<'a, X> for BorrowMut
{
  type Applied = &'a mut X;
}

impl TypeAppGenericUnsized for BorrowMut
{
  fn with_type_app<'a, X: 'a + ?Sized, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

impl TypeCon for Own {}

impl<'a, X: 'a + ?Sized> TypeApp<'a, X> for Own
{
  type Applied = Box<X>;
}

impl TypeAppGenericUnsized for Own
{
  fn with_type_app<'a, X: 'a + ?Sized, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

impl IsRef for Own
{
  fn get_ref<'a, 'b, X: 'a + ?Sized>(x: &'b App<'a, Self, X>) -> &'b X
  where
    'a: 'b,
  {
    x.0.get_applied_borrow()
  }
}

impl IsMutRef for Own
{
  fn get_mut_ref<'a, 'b, X: 'a + ?Sized>(
    x: &'b mut App<'a, Self, X>
  ) -> &'b mut X
  where
    'a: 'b,
  {
    x.get_applied_borrow_mut()
  }
}

impl IsOwn for Own
{
  fn get_own<'a, X: 'a>(x: App<'a, Self, X>) -> X
  {
    *x.get_applied()
  }
}

impl CloneApp for Own
{
  fn clone_app<'a, X: 'a>(fx: &App<'a, Self, X>) -> App<'a, Self, X>
  where
    X: Clone,
  {
    wrap_app(fx.get_applied_borrow().clone())
  }
}

impl CloneApp for BoxF
{
  fn clone_app<'a, X: 'a>(fx: &App<'a, Self, X>) -> App<'a, Self, X>
  where
    X: Clone,
  {
    wrap_app(fx.get_applied_borrow().clone())
  }
}

impl TypeCon for BoxF {}

impl<'a, X: 'a + ?Sized> TypeApp<'a, X> for BoxF
{
  type Applied = Box<X>;
}

impl TypeAppGenericUnsized for BoxF
{
  fn with_type_app<'a, X: 'a + ?Sized, R: 'a, Cont: 'a>(cont: Box<Cont>) -> R
  where
    Self: 'a,
    Cont: TypeAppCont<'a, Self, X, R>,
  {
    cont.on_type_app()
  }
}

impl IsRef for BoxF
{
  fn get_ref<'a, 'b, X: 'a + ?Sized>(x: &'b App<'a, Self, X>) -> &'b X
  where
    'a: 'b,
  {
    x.0.get_applied_borrow()
  }
}

impl IsMutRef for BoxF
{
  fn get_mut_ref<'a, 'b, X: 'a + ?Sized>(
    x: &'b mut App<'a, Self, X>
  ) -> &'b mut X
  where
    'a: 'b,
  {
    x.get_applied_borrow_mut()
  }
}

impl IsOwn for BoxF
{
  fn get_own<'a, X: 'a>(x: App<'a, Self, X>) -> X
  {
    *x.get_applied()
  }
}

impl IsBox for BoxF
{
  fn get_box<'a, X: 'a + ?Sized>(x: App<'a, Self, X>) -> Box<X>
  {
    x.get_applied()
  }
}
