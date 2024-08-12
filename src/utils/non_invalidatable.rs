//! Types that have no invalid states

use std::mem::ManuallyDrop;

/// A trait for types that can't be invalid.
///
/// # Safety
///
/// The implemeter must guarantee that the implemented type is valid.
pub unsafe trait NonInvalidatable<const SIZE: usize> {}

/// A way to quickly implement [`NonInvalidatable`] for types.
macro_rules! impls {
    (unsafe { $($t:ty)* }) => {
        $(
            // SAFETY: the macro user thinks it's safe
            unsafe impl NonInvalidatable<{ size_of::<$t>() }> for $t {}
        )*
    };
}

impls!(
    unsafe {
        u8 u16 u32 u64 u128
        i8 i16 i32 i64 i128
        f32 f64
    }
);

/// Used to cast between 2 different types
union Caster<T, U> {
    /// First type
    t: ManuallyDrop<T>,
    /// Second type
    u: ManuallyDrop<U>,
}

/// Safely transmutes a [`NonInvalidatable`] type into another.
pub const fn transmute<
    Src: NonInvalidatable<SIZE>,
    Dst: NonInvalidatable<SIZE>,
    const SIZE: usize,
>(
    src: Src,
) -> Dst {
    // SAFETY: the types can't be invalid
    unsafe {
        ManuallyDrop::into_inner(
            Caster {
                t: ManuallyDrop::new(src),
            }
            .u,
        )
    }
}
