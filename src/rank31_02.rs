use crate::{rank32_11, rank32_11_unchecked, Ranks};

#[inline(always)]
pub fn rank31_02(i0: usize) -> &'static Ranks {
    rank32_11(i0)
}

/// # Safety
/// 0 <= i0 < 52
#[inline(always)]
pub unsafe fn rank31_02_unchecked(i0: usize) -> &'static Ranks {
    rank32_11_unchecked(i0)
}
