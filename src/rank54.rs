use crate::{rank53_01, rank53_02, rank53_11, Ia};

#[inline(always)]
pub fn rank54_11(i0: usize, i1: usize, i2: usize) -> &'static [u64] {
    rank53_02(i0, i1, i2)
}

#[inline(always)]
pub fn rank54_10(i0: usize, i1: usize, i2: usize) -> Ia<u64> {
    rank53_01(i0, i1, i2)
}

#[inline(always)]
pub fn rank54_20(i0: usize, i1: usize) -> Ia<u64> {
    rank53_11(i0, i1)
}
