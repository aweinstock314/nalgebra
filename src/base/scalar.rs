use std::any::Any;
use std::any::TypeId;
use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;

/// The basic scalar type for all structures of `nalgebra`.
///
/// This does not make any assumption on the algebraic properties of `Self`.
pub trait Scalar: PartialEq + Debug + Any {
    #[inline]
    /// Tests if `Self` the same as the type `T`
    ///
    /// Typically used to test of `Self` is a f32 or a f64 with `N::is::<f32>()`.
    fn is<T: Scalar>() -> bool {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }
}
impl<T: Copy + PartialEq + Debug + Any> Scalar for T {}

/// A newtype wrapper that provides the necessary trait implementations for MaybeUninit
/// This is needed because MaybeUninit does not pass through implementations on uninitialized data (for good reason)
pub struct UninitScalar<S: Scalar>(MaybeUninit<S>);

impl<S: Scalar> Clone for UninitScalar<S> {
    fn clone(&self) -> Self {
        panic!("Clone::clone called on potentially uninitialized data")
    }
}

impl<S: Scalar> PartialEq for UninitScalar<S> {
    fn eq(&self, _: &Self) -> bool {
        panic!("PartialEq::eq called on potentially uninitalized data")
    }
}
impl<S: Scalar> Debug for UninitScalar<S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "UninitScalar")
    }
}
impl<S: Scalar> Scalar for UninitScalar<S> {}
