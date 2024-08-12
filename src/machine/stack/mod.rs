//! A stack.
//!
//! Read the documentation for [`Stack`] for more info.

pub mod stackoverflow;

use std::{fmt, ptr};

use stackoverflow::StackOverflow;

use crate::utils::array_debug::DebugArray;

/// A stack.
///
/// A stack is a kind of memory that can only be manipulated by
/// pushing (appending) an element, or by popping (removing) the last element,
/// like a stack of plates, which makes it a **first in, last out (LOFI) ** data type.
///
///
#[derive(Clone)]
pub struct Stack {
    /// The data storage of the stack.
    pub vec: Vec<u8>,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            vec: Vec::with_capacity(4095),
        }
    }
}

impl Stack {
    /// Returns the capacity of the stack (how big it is) in bytes.
    #[inline]
    #[must_use]
    pub fn total_space(&self) -> usize {
        self.vec.capacity()
    }
    /// Returns how much space of the stack has been used in bytes.
    #[inline]
    #[must_use]
    pub fn used_space(&self) -> usize {
        self.vec.len()
    }
    /// Returns how much space is left of the stack in bytes.
    #[inline]
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub fn space_left(&self) -> usize {
        self.total_space() - self.used_space()
    }

    /// Sets how much space is used of the stack.
    ///
    /// # Safety
    ///
    /// Any value of type [`u8`] can't possibly be invalid.
    ///
    /// The caller must guarantee that `new_len` doesn't exceed the capacity of the stack.
    #[inline]
    #[warn(warnings)]
    pub unsafe fn set_used_space(&mut self, new_len: usize) {
        // SAFETY: doc comment above
        unsafe {
            self.vec.set_len(new_len);
        }
    }

    /// Pushes a byte onto the [`Stack`].
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`] if the stack has no more space (`capacity == length`).
    pub fn push_byte(&mut self, byte: u8) -> Result<(), StackOverflow> {
        if self.space_left() == 0 {
            return Err(StackOverflow);
        }
        self.vec.push(byte);
        Ok(())
    }
    /// Pops a byte from the [`Stack`].
    ///
    /// Returns [`None`] if there are no bytes on the [`Stack`].
    pub fn pop_byte(&mut self) -> Option<u8> {
        self.vec.pop()
    }

    /// Copies a slice onto the [`Stack`].
    ///
    /// This is done by allocating `bytes` bytes and writing the slice onto the buffer using [`ptr::copy`]
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`] and doesn't allocate if the stack doesn't have enough space left.
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), StackOverflow> {
        let len = self.used_space();
        let bytes_len = bytes.len();

        self.alloc(bytes_len)?;

        // SAFETY: line above checked that it doesn't overflow
        let dst = unsafe { self.vec.as_mut_ptr().add(len) };

        // SAFETY: comment above
        unsafe {
            ptr::copy(bytes.as_ptr(), dst, bytes_len);
        }

        Ok(())
    }

    /// Pushes `bytes` bytes onto the [`Stack`].
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`] and doesn't allocate if the bytes don't fit on the stack (`bytes > capacity - length`).
    pub fn alloc(&mut self, bytes: usize) -> Result<(), StackOverflow> {
        if bytes > self.space_left() {
            return Err(StackOverflow);
        }
        for _ in 0..bytes {
            self.vec.push(0);
        }
        Ok(())
    }
    /// Pops `bytes` bytes from the [`Stack`].
    ///
    /// Pops the amount of bytes specified and returns a slice of them.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`] if there is not enough space.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the slice isn't used when
    /// the [`Stack`] is pushed onto again to prevent race conditions.
    pub unsafe fn dealloc(&mut self, bytes: usize) -> Result<(), StackOverflow> {
        let len = self.used_space();
        if bytes > len {
            Err(StackOverflow)
        } else {
            #[allow(clippy::arithmetic_side_effects)]
            let new_len = self.used_space() - bytes;

            // SAFETY: overflows checked above
            unsafe {
                self.set_used_space(new_len);
            }

            Ok(())
        }
    }
    /// Pops a 16-bit big endian unsigned integer from the stack.
    pub fn pop_u16(&mut self) -> Option<u16> {
        let mut array = [0, 0];

        array[1] = self.pop_byte()?;
        array[0] = self.pop_byte()?;

        Some(u16::from_be_bytes(array))
    }
    /// Pops a 32-bit big endian unsigned integer from the stack.
    pub fn pop_u32(&mut self) -> Option<u32> {
        let mut array = [0, 0, 0, 0];

        array[3] = self.pop_byte()?;
        array[2] = self.pop_byte()?;
        array[1] = self.pop_byte()?;
        array[0] = self.pop_byte()?;

        Some(u32::from_be_bytes(array))
    }
    /// Pops a 64-bit big endian unsigned integer from the stack.
    pub fn pop_u64(&mut self) -> Option<u64> {
        let mut array = [0, 0, 0, 0, 0, 0, 0, 0];

        array[7] = self.pop_byte()?;
        array[6] = self.pop_byte()?;
        array[5] = self.pop_byte()?;
        array[4] = self.pop_byte()?;
        array[3] = self.pop_byte()?;
        array[2] = self.pop_byte()?;
        array[1] = self.pop_byte()?;
        array[0] = self.pop_byte()?;

        Some(u64::from_be_bytes(array))
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::indexing_slicing)]
        let last = self.used_space().checked_sub(16).map(|n| &self.vec[n..]);

        f.write_fmt(format_args!(
            "{}/{} {:?}",
            self.used_space(),
            self.total_space(),
            &DebugArray::debug(self.vec.get(0..16).unwrap_or(&[]), true, last,)
        ))
    }
}
