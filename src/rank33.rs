use crate::rank32_00::{rank32_00, rank32_00_unchecked};
use crate::rank32_01::{rank32_01, rank32_01_unchecked};
use crate::rank32_11::{rank32_11, rank32_11_unchecked};

#[inline(always)]
pub fn rank33_0(i0: usize, i1: usize, i2: usize) -> u64 {
    let ranks = rank32_00(i1, i2);
    ranks[i0]
}

/// # Safety
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank33_0_unchecked(i0: usize, i1: usize, i2: usize) -> u64 {
    let ranks = rank32_00_unchecked(i1, i2);
    *ranks.get_unchecked(i0)
}

#[inline(always)]
pub fn rank33_1(i0: usize, i1: usize) -> &'static [u64; 14] {
    rank32_01(i0, i1)
}

/// # Safety
/// 0 <= i0 < i1 < 52
#[inline(always)]
pub unsafe fn rank33_1_unchecked(i0: usize, i1: usize) -> &'static [u64; 14] {
    rank32_01_unchecked(i0, i1)
}

#[inline(always)]
pub fn rank33_2(i0: usize) -> &'static [u64; 92] {
    rank32_11(i0)
}

/// # Safety
/// 0 <= i0 < 52
#[inline(always)]
pub unsafe fn rank33_2_unchecked(i0: usize) -> &'static [u64; 92] {
    rank32_11_unchecked(i0)
}
