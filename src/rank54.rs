use crate::{
    rank53_01, rank53_01_unchecked, rank53_02, rank53_02_unchecked, rank53_11, rank53_11_unchecked,
    Ia,
};

#[inline(always)]
pub fn rank54_11(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank53_02(i0, i1, i2)
}

/// # Safty
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank54_11_unchecked(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank53_02_unchecked(i0, i1, i2)
}

#[inline(always)]
pub fn rank54_10(i0: usize, i1: usize, i2: usize) -> Ia<u64> {
    rank53_01(i0, i1, i2)
}

/// # Safty
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank54_10_unchecked(i0: usize, i1: usize, i2: usize) -> Ia<u64> {
    rank53_01_unchecked(i0, i1, i2)
}

#[inline(always)]
pub fn rank54_20(i0: usize, i1: usize) -> Ia<u64> {
    rank53_11(i0, i1)
}

/// # Safty
/// 0 <= i0 < i1 < 52
#[inline(always)]
pub unsafe fn rank54_20_unchecked(i0: usize, i1: usize) -> Ia<u64> {
    rank53_11_unchecked(i0, i1)
}
