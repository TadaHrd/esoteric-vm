//! A nice way to debug arrays without filling the console.
//!
//! Read the docs for [`DebugArray`] for more info.

use core::fmt::{Debug, Formatter};

/// A nice way to debug arrays without filling the console.
///
/// This type isn't actually given to the library user,
/// instead a [`DebugArrayDebugger`] is used to prevent
/// printing on multiple lines.
///
/// Examples at [`DebugArray::debug`] and [`ArrayDebug::array_debug`].
#[derive(Clone, Copy)]
pub struct DebugArray<'a, T: Debug>(&'a [T], bool, Option<&'a [T]>);

impl<'a, T: Debug> DebugArray<'a, T> {
    /// Make a [`DebugArrayDebugger`] to nicely debug.
    ///
    /// `array` is the first part of the array to debug,\
    /// `non_exhaustive` is whether or not `..` should be
    /// printed after the first part,\
    /// `continuation` is the optional second part of the
    /// array to debug, if it's present, `non_exhaustive`
    /// is treated as true.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use esoteric_vm::::DebugArray;
    /// # use std::fmt;
    /// struct MyType {
    ///     array: [u8; 256]
    /// }
    ///
    /// impl fmt::Debug for MyType {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         f.debug_struct("MyType")
    ///             .field("array", &DebugArray::debug(&self.array[0..16], true, &self.array[self.array.len() - 16..]))
    ///             .finish()
    ///     }
    /// }
    /// ```
    pub const fn debug(
        array: &'a [T],
        non_exhaustive: bool,
        continuation: Option<&'a [T]>,
    ) -> DebugArrayDebugger<'a, T> {
        DebugArrayDebugger(Self(array, non_exhaustive, continuation))
    }
}

impl<'a, T: Debug> Debug for DebugArray<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = f.debug_list();
        let mut ret = ret.entries(self.0);

        if self.1 || self.2.is_some() {
            ret = ret.entry(&..);
        }
        if let Some(v) = self.2 {
            ret = ret.entries(v);
        }

        ret.finish()
    }
}

/// A wrapper type for [`DebugArray`].
///
/// This type is used to prevent printing on multiple lines.
///
/// A value of this type is obtained by calling
/// [`DebugArray::debug`] or [`ArrayDebug::array_debug`].
#[repr(transparent)]
pub struct DebugArrayDebugger<'a, T: Debug>(DebugArray<'a, T>);

impl<'a, T: Debug> Debug for DebugArrayDebugger<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

/// A trait for debugging arrays without filling the console.
///
/// Examples as [`ArrayDebug::array_debug`] and [`ArrayDebug::into_debug_array`].
pub trait ArrayDebug<'a, T: Debug>
where
    Self: Sized,
{
    /// Turns the array into a [`DebugArray`].
    ///
    /// This is done by taking `&self[..first_elems]`, potentially suffixing it with `..` (explained down below),
    /// and potentially adding `&self[(self.len() - last_elems)..]` to the end.
    ///
    /// If `first_elems` is less than the length of the array, the first elements will be followed by `..` as it's non-exhaustive.
    /// If `last_elems` is not zero, there will be `..` and they'll be followed by the last elements.
    ///
    /// Note that the input parameters may exceed the length of the array, in which case it must be handled.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use crate::esoteric_vm::utils::array_debug::ArrayDebug;
    /// let slice = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    ///
    /// assert_eq!(&format!("{:?}", slice.array_debug(4, 4)), "[0, 1, 2, 3, .., 12, 13, 14, 15]")
    /// ```
    fn into_debug_array(self, first_elems: usize, last_elems: usize) -> DebugArray<'a, T>;
    /// Turns the array into a [`DebugArrayDebugger`].
    ///
    /// This is done by calling [`ArrayDebug::into_debug_array`] on `self`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use esoteric_vm::utils::array_debug::ArrayDebug;
    /// let slice = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    ///
    /// assert_eq!(&format!("{:?}", slice.array_debug(4, 4)), "[0, 1, 2, 3, .., 12, 13, 14, 15]")
    /// ```
    fn array_debug(self, first_elems: usize, last_elems: usize) -> DebugArrayDebugger<'a, T> {
        DebugArrayDebugger(self.into_debug_array(first_elems, last_elems))
    }
}

impl<'a, T: Debug> ArrayDebug<'a, T> for &'a [T] {
    #[allow(clippy::indexing_slicing)]
    fn into_debug_array(self, mut first_elems: usize, mut last_elems: usize) -> DebugArray<'a, T> {
        let len = self.len();

        if last_elems > len {
            last_elems = 0;
        }
        let non_exhaustive = if first_elems >= len {
            first_elems = len;
            last_elems = 0;

            false
        } else {
            true
        };

        let first_elems = &self[..first_elems];
        #[allow(clippy::arithmetic_side_effects)]
        let last_elems = match &self[len - last_elems..] {
            &[] => None,
            v => Some(v),
        };

        DebugArray(first_elems, non_exhaustive, last_elems)
    }
}
