use crate::rank32_00::rank32_00;
use crate::rank32_01::rank32_01;
use crate::rank32_11::rank32_11;
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

#[inline(always)]
pub fn rank33_1(i0: usize, i1: usize) -> &'static [u64; 14] {
    rank32_01(i0, i1)
}

#[inline(always)]
pub fn rank33_2(i0: usize) -> &'static [u64; 92] {
    rank32_11(i0)
}
