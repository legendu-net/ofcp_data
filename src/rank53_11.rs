use crate::{rank53_20, rank53_20_unchecked, Ranks};

#[inline(always)]
pub fn rank53_11(i0: usize, i1: usize) -> &'static [&'static Ranks] {
    let ranks = rank53_20(i0);
    let begin = i1 * 52;
    &ranks[begin..begin + 52]
}

/// # Safety
/// 0 <= i0 < i1 < 52
#[inline(always)]
pub unsafe fn rank53_11_unchecked(i0: usize, i1: usize) -> &'static [&'static Ranks] {
    let ranks = rank53_20_unchecked(i0);
    let begin = i1 * 52;
    ranks.get_unchecked(begin..begin + 52)
}
