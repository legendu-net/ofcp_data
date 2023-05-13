use crate::rank32_20;

#[inline(always)]
pub fn rank32_11(i0: usize) -> &'static [u64] {
    rank32_20()[i0]
}

/// # Safety
/// 0 <= i0 < 52
#[inline(always)]
pub unsafe fn rank32_11_unchecked(i0: usize) -> &'static [u64] {
    rank32_20().get_unchecked(i0)
}
