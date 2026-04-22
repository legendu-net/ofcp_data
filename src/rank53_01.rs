use crate::{rank53_10, rank53_10_unchecked, Ranks};

#[inline(always)]
pub fn rank53_01(i0: usize, i1: usize, i2: usize) -> &'static [&'static Ranks] {
    let ranks = rank53_10(i0, i1);
    let begin = i2 * 52;
    &ranks[begin..begin + 52]
}

/// # Safety
/// 0 <= i0 < i1 < i2 < 52
#[inline(always)]
pub unsafe fn rank53_01_unchecked(i0: usize, i1: usize, i2: usize) -> &'static [&'static Ranks] { unsafe {
    let ranks = rank53_10_unchecked(i0, i1);
    let begin = i2 * 52;
    ranks.get_unchecked(begin..begin + 52)
}}
