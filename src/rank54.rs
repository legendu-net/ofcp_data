use crate::{
    rank53_00, rank53_00_unchecked, rank53_01, rank53_01_unchecked, rank53_02, rank53_02_unchecked,
    rank53_11, rank53_11_unchecked, Ia,
};

#[inline(always)]
pub fn rank54_00(i0: usize, i1: usize, i2: usize, i3: usize) -> &'static [u64] {
    let begin = i3 * 52;
    &rank53_00(i0, i1, i2)[begin..begin + 52]
}

/// # Safety
/// 0 <= i0 < i1 < i2 < i3 < 52
#[inline(always)]
pub unsafe fn rank54_00_unchecked(i0: usize, i1: usize, i2: usize, i3: usize) -> &'static [u64] {
    let begin = i3 * 52;
    rank53_00_unchecked(i0, i1, i2).get_unchecked(begin..begin + 52)
}

#[inline(always)]
pub fn rank54_01(i0: usize, i1: usize, i2: usize, i3: usize) -> &'static [u64] {
    rank53_01(i0, i1, i2).values(i3)
}

/// # Safety
/// 0 <= i0 < i1 < i2 < i3 < 52
#[inline(always)]
pub unsafe fn rank54_01_unchecked(i0: usize, i1: usize, i2: usize, i3: usize) -> &'static [u64] {
    rank53_01_unchecked(i0, i1, i2).values_unchecked(i3)
}

#[inline(always)]
pub fn rank54_11(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank53_02(i0, i1, i2)
}

/// # Safety
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank54_11_unchecked(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank53_02_unchecked(i0, i1, i2)
}

#[inline(always)]
pub fn rank54_10(i0: usize, i1: usize, i2: usize) -> Ia<u64> {
    rank53_01(i0, i1, i2)
}

/// # Safety
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank54_10_unchecked(i0: usize, i1: usize, i2: usize) -> Ia<u64> {
    rank53_01_unchecked(i0, i1, i2)
}

#[inline(always)]
pub fn rank54_20(i0: usize, i1: usize) -> Ia<u64> {
    rank53_11(i0, i1)
}

/// # Safety
/// 0 <= i0 < i1 < 52
#[inline(always)]
pub unsafe fn rank54_20_unchecked(i0: usize, i1: usize) -> Ia<u64> {
    rank53_11_unchecked(i0, i1)
}
