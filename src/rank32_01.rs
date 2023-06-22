use crate::{rank32_10, rank32_10_unchecked, Ranks};

#[inline(always)]
pub fn rank32_01(i0: usize, i1: usize) -> &'static Ranks {
    rank32_10(i0)[i1]
}

/// # Safety
/// 0 <= i0 < i1 < 52
#[inline(always)]
pub unsafe fn rank32_01_unchecked(i0: usize, i1: usize) -> &'static Ranks {
    rank32_10_unchecked(i0).get_unchecked(i1)
}
