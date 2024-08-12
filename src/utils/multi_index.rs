//! A way to index a byte slice to get more
//! than a byte with 16-bit addressing.

/// A way to index a byte slice to get more
/// than a byte with 16-bit addressing.
///
/// This is big endian.
#[must_use]
#[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
pub fn index_u64(slice: &[u8], idx: u16) -> u64 {
    debug_assert!(slice.len() + idx as usize >= 8);

    let mut ret = [0; 8];

    ret[7] = slice[idx as usize];
    ret[6] = slice[idx.wrapping_add(1) as usize];
    ret[5] = slice[idx.wrapping_add(2) as usize];
    ret[4] = slice[idx.wrapping_add(3) as usize];
    ret[3] = slice[idx.wrapping_add(4) as usize];
    ret[2] = slice[idx.wrapping_add(5) as usize];
    ret[1] = slice[idx.wrapping_add(6) as usize];
    ret[0] = slice[idx.wrapping_add(7) as usize];

    u64::from_be_bytes(ret)
}
