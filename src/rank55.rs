use crate::{
    rank54_00, rank54_00_unchecked, rank54_01, rank54_01_unchecked, rank54_11, rank54_11_unchecked,
    VALID_RANK_CUTOFF,
};

#[inline(always)]
pub fn rank55_0(i0: usize, i1: usize, i2: usize, i3: usize, i4: usize) -> u64 {
    let cache = rank54_00(i1, i2, i3, i4);
    let r = cache[i0];
    if r > VALID_RANK_CUTOFF {
        r
    } else {
        cache[r as usize]
    }
}

/// # Safety
/// 0 <= i0 < i1 < i2 < i3 < i4 < 52
#[inline(always)]
pub unsafe fn rank55_0_unchecked(i0: usize, i1: usize, i2: usize, i3: usize, i4: usize) -> u64 {
    let cache = rank54_00_unchecked(i1, i2, i3, i4);
    let r = *cache.get_unchecked(i0);
    if r > VALID_RANK_CUTOFF {
        r
    } else {
        *cache.get_unchecked(r as usize)
    }
}

#[inline(always)]
pub fn rank55_1(i0: usize, i1: usize, i2: usize, i3: usize) -> &'static [u64] {
    rank54_01(i0, i1, i2, i3)
}

/// # Safety
/// 0 <= i0 < i1 < i2 < i3 < 52
#[inline(always)]
pub unsafe fn rank55_1_unchecked(i0: usize, i1: usize, i2: usize, i3: usize) -> &'static [u64] {
    rank54_01_unchecked(i0, i1, i2, i3)
}

#[inline(always)]
pub fn rank55_2(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank54_11(i0, i1, i2)
}

/// # Safety
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank55_2_unchecked(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank54_11_unchecked(i0, i1, i2)
}
