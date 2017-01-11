use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};


/// A `TypeId` combined with a phantom lifetime, to preserve whatever lifetime the initial data
/// might have had.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BoundedTypeId<'a>(TypeId, PhantomData<&'a ()>);


impl<'a> BoundedTypeId<'a> {
    /// Get the `BoundedTypeId` of a non-'static type.
    pub fn of<T: 'a + AsStatic>() -> BoundedTypeId<'a> {
        BoundedTypeId(TypeId::of::<T::Static>(), PhantomData)
    }
}


/// Represents an immutable reference to a type which can be cast to a `'static` lifetime and then
/// used with `Any`. Supports similar operations to `&Any`.
#[derive(Clone, Copy, Debug)]
pub struct BoundedAnyRef<'a>(&'a Any, TypeId);


impl<'a, T: 'a + AsStatic> From<&'a T> for BoundedAnyRef<'a>
    where T::Static: Sized
{
    fn from(t_ref: &'a T) -> BoundedAnyRef<'a> {
        BoundedAnyRef(unsafe { &*(t_ref as *const T as *const T::Static) },
                      TypeId::of::<T::Static>())
    }
}


impl<'a> BoundedAnyRef<'a> {
    /// Check whether the underlying type is a `T`, disregarding lifetimes.
    pub fn is<T: 'a + AsStatic>(&self) -> bool
        where T::Static: Sized
    {
        self.0.is::<T::Static>()
    }


    /// Try to downcast to a reference to the correctly-lifetimed `T`.
    pub fn downcast_ref<T: 'a + AsStatic>(&self) -> Option<&'a T>
        where T::Static: Sized
    {
        unsafe {
            self.0
                .downcast_ref::<T::Static>()
                .map(|opt_static| &*(opt_static as *const T::Static as *const T))
        }
    }
}


/// Represents a mutable reference to a type which can be cast to a `'static` lifetime and then
/// used with `Any`. Supports similar operations to `&mut Any`.
#[derive(Debug)]
pub struct BoundedAnyMut<'a>(&'a mut Any, TypeId);


impl<'a, T: 'a + AsStatic> From<&'a mut T> for BoundedAnyMut<'a>
    where T::Static: Sized
{
    fn from(t_ref: &'a mut T) -> BoundedAnyMut<'a> {
        BoundedAnyMut(unsafe { &mut *(t_ref as *mut T as *mut T::Static) },
                      TypeId::of::<T::Static>())
    }
}


impl<'a> BoundedAnyMut<'a> {
    /// Check whether the underlying type is a `T`, disregarding lifetimes. This operation is
    /// duplicated from `BoundedAnyRef` since there does not appear to be a way to hook into Rust's
    /// pointer weakening coercions.
    pub fn is<T: 'a + AsStatic>(&self) -> bool
        where T::Static: Sized
    {
        self.0.is::<T::Static>()
    }


    /// Try to downcast to a reference to the correctly-lifetimed `T`. This operation is
    /// duplicated from `BoundedAnyRef` since there does not appear to be a way to hook into Rust's
    /// pointer weakening coercions.
    pub fn downcast_ref<T: 'a + AsStatic>(&self) -> Option<&'a T>
        where T::Static: Sized
    {
        unsafe {
            self.0
                .downcast_ref::<T::Static>()
                .map(|opt_static| &*(opt_static as *const T::Static as *const T))
        }
    }


    /// Try to downcast to a mutable reference to the correctly-lifetimed `T`.
    pub fn downcast_mut<T: 'a + AsStatic>(&mut self) -> Option<&'a mut T>
        where T::Static: Sized
    {
        unsafe {
            self.0
                .downcast_mut::<T::Static>()
                .map(|opt_static| &mut *(opt_static as *mut T::Static as *mut T))
        }
    }
}


pub unsafe trait AsStatic {
    type Static: ?Sized + Any + 'static;
}


unsafe impl<'a, T: AsStatic> AsStatic for &'a T {
    type Static = &'static T::Static;
}


unsafe impl<'a, T: AsStatic> AsStatic for &'a mut T {
    type Static = &'static mut T::Static;
}


unsafe impl<T: AsStatic> AsStatic for [T]
    where T::Static: Sized
{
    type Static = [T::Static];
}


unsafe impl<T: AsStatic> AsStatic for Box<T> {
    type Static = Box<T::Static>;
}


unsafe impl<T: AsStatic> AsStatic for Vec<T>
    where T::Static: Sized
{
    type Static = Vec<T::Static>;
}


macro_rules! trivial {
    ($($t:ty),+) => {
        $(
            unsafe impl AsStatic for $t {
                type Static = $t;
            }
        )+
    }
}
trivial!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
trivial!(str, String);
trivial!(bool, ());


unsafe impl<A: AsStatic> AsStatic for (A,)
    where A::Static: Sized
{
    type Static = (A::Static);
}


unsafe impl<A: AsStatic, B: AsStatic> AsStatic for (A, B)
    where A::Static: Sized,
          B::Static: Sized
{
    type Static = (A::Static, B::Static);
}


unsafe impl<A: AsStatic, B: AsStatic, C: AsStatic> AsStatic for (A, B, C)
    where A::Static: Sized,
          B::Static: Sized,
          C::Static: Sized
{
    type Static = (A::Static, B::Static, C::Static);
}


unsafe impl<A: AsStatic, B: AsStatic, C: AsStatic, D: AsStatic> AsStatic for (A, B, C, D)
    where A::Static: Sized,
          B::Static: Sized,
          C::Static: Sized,
          D::Static: Sized
{
    type Static = (A::Static, B::Static, C::Static, D::Static);
}


unsafe impl<A: AsStatic, B: AsStatic, C: AsStatic, D: AsStatic, E: AsStatic> AsStatic
    for (A, B, C, D, E)
    where A::Static: Sized,
          B::Static: Sized,
          C::Static: Sized,
          D::Static: Sized,
          E::Static: Sized
{
    type Static = (A::Static, B::Static, C::Static, D::Static, E::Static);
}


macro_rules! array {
    ($($l:expr),+) => {
        $(
            unsafe impl<T: AsStatic> AsStatic for [T; $l] where T::Static: Sized {
                type Static = [T::Static; $l];
            }
        )+
    }
}
array!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
array!(18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);


unsafe impl<T: AsStatic> AsStatic for Rc<T> {
    type Static = Rc<T::Static>;
}


unsafe impl<T: AsStatic> AsStatic for Weak<T> {
    type Static = Weak<T::Static>;
}
