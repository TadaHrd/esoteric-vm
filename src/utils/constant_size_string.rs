//! A constantly-sized string.
//!
//! More info at [`ConstantSizeString`].

use core::str;
use std::{error::Error, fmt, ptr};

/// A string with a constant capacity.
///
/// This is useful when you want string that doesn't exceed
/// a certain capacity but can shrink and grow in length.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstantSizeString {
    /// The inner vector.
    pub vec: Vec<u8>,
}

impl ConstantSizeString {
    /// Make a new [`ConstantSizeString`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `vec` is valid UTF-8.
    #[inline]
    #[must_use]
    pub unsafe fn new(vec: Vec<u8>) -> Self {
        Self { vec }
    }
    /// Pushes a byte onto the [`ConstantSizeString`].
    ///
    /// If there is available space, it pushes the byte,
    /// Returns [`Overflow`] if there is no more available space.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `byte` is valid UTF-8.
    pub unsafe fn push_byte(&mut self, byte: u8) -> Result<(), Overflow> {
        let len = self.vec.len();
        if len < self.vec.capacity() {
            // SAFETY: We just checked that `len` doesn't exceed the capacity
            let ptr = unsafe { self.vec.as_mut_ptr().add(len) };

            // SAFETY: ptr is valid as stated above
            unsafe {
                *ptr = byte;
            }

            // SAFETY: first safety comment
            unsafe {
                #[allow(clippy::arithmetic_side_effects)]
                self.vec.set_len(len + 1);
            }
            Ok(())
        } else {
            Err(Overflow)
        }
    }
    /// Pushes bytes onto the [`ConstantSizeString`].
    ///
    /// If there is enough available space, it pushes the byte,
    /// Returns [`Overflow`] if there is not enough available space.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `bytes` are valid UTF-8.
    pub unsafe fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), Overflow> {
        #[allow(clippy::arithmetic_side_effects)]
        let len = self.vec.len() + bytes.len();
        if len > self.vec.capacity() {
            Err(Overflow)
        } else {
            // SAFETY: It cannot exceed the bounds of the slice (cap) because it's checked above
            let ptr = unsafe { self.vec.as_mut_ptr().add(self.vec.len()) };

            // SAFETY: ptr is valid as stated above
            unsafe {
                ptr::copy(bytes.as_ptr(), ptr, bytes.len());
            }

            // SAFETY: first safety comment
            unsafe {
                self.vec.set_len(len);
            }

            Ok(())
        }
    }

    /// Removes and returns the last byte,
    /// returning [`None`] if there are none left.
    #[inline]
    pub fn pop_byte(&mut self) -> Option<u8> {
        self.vec.pop()
    }

    /// Empties the string.
    #[inline]
    pub fn clear(&mut self) {
        // SAFETY: there are no uninitialized elements
        unsafe { self.vec.set_len(0) }
    }

    /// Gets the length of the string.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Checks if the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets a byte from the string.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u8> {
        self.vec.get(index).copied()
    }
    /// Sets a byte in the string.
    pub fn set(&mut self, index: usize, value: u8) -> Result<(), Overflow> {
        self.vec.get_mut(index).map_or(Err(Overflow), |v| {
            *v = value;
            Ok(())
        })
    }
}

impl fmt::Debug for ConstantSizeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::expect_used)]
        fmt::Debug::fmt(
            str::from_utf8(&self.vec).expect("invalid `ConstantSizeString` print attempts"),
            f,
        )
    }
}

impl fmt::Display for ConstantSizeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::expect_used)]
        f.write_str(str::from_utf8(&self.vec).expect("invalid `ConstantSizeString` print attempts"))
    }
}

/// An overflow.
///
/// Returned if no more bytes fit on the string
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Overflow;

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Overflow")
    }
}

impl Error for Overflow {}
