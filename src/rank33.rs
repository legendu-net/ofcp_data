use crate::rank32_00::{rank32_00, rank32_00_unchecked};
use crate::rank32_01::{rank32_01, rank32_01_unchecked};
use crate::rank32_11::{rank32_11, rank32_11_unchecked};
use crate::VALID_RANK_CUTOFF;

#[inline(always)]
pub fn rank33_0(i0: usize, i1: usize, i2: usize) -> u64 {
    let cache = rank32_00(i1, i2);
    let r = cache[i0];
    if r > VALID_RANK_CUTOFF {
        r
    } else {
        cache[r as usize]
    }
}

/// # Safety
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank33_0_unchecked(i0: usize, i1: usize, i2: usize) -> u64 {
    let cache = rank32_00_unchecked(i1, i2);
    let r = *cache.get_unchecked(i0);
    if r > VALID_RANK_CUTOFF {
        r
    } else {
        *cache.get_unchecked(r as usize)
    }
}

#[inline(always)]
pub fn rank33_1(i0: usize, i1: usize) -> &'static [u64; 14] {
    rank32_01(i0, i1)
}

/// # Safty
/// 0 <= i0 < i1 < 52
#[inline(always)]
pub unsafe fn rank33_1_unchecked(i0: usize, i1: usize) -> &'static [u64; 14] {
    rank32_01_unchecked(i0, i1)
}

#[inline(always)]
pub fn rank33_2(i0: usize) -> &'static [u64; 92] {
    rank32_11(i0)
}

/// # Safty
/// 0 <= i0 < 52
#[inline(always)]
pub unsafe fn rank33_2_unchecked(i0: usize) -> &'static [u64; 92] {
    rank32_11_unchecked(i0)
}
