use crate::qljs_assert;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_feature = "sse2")]
pub struct CharVector16SSE2(__m128i);

#[cfg(target_feature = "sse2")]
impl CharVector16SSE2 {
    // data must point to at least 16 elements.
    pub fn load(data: &[u8]) -> CharVector16SSE2 {
        qljs_assert!(data.len() >= 16);
        unsafe { CharVector16SSE2(_mm_loadu_si128(data.as_ptr() as *const __m128i)) }
    }

    pub fn repeated(x: u8) -> CharVector16SSE2 {
        unsafe { CharVector16SSE2(_mm_set1_epi8(x as i8)) }
    }

    // out_data must point to at least 16 elements.
    pub fn store(&self, out_data: &mut [u8]) {
        qljs_assert!(out_data.len() >= 16);
        unsafe {
            _mm_storeu_si128(out_data.as_mut_ptr() as *mut __m128i, self.0);
        }
    }

    pub fn lane_eq(&self, rhs: CharVector16SSE2) -> BoolVector16SSE2 {
        unsafe { BoolVector16SSE2(_mm_cmpeq_epi8(self.0, rhs.0)) }
    }

    pub fn lane_lt(&self, rhs: CharVector16SSE2) -> BoolVector16SSE2 {
        unsafe { BoolVector16SSE2(_mm_cmplt_epi8(self.0, rhs.0)) }
    }

    pub fn lane_gt(&self, rhs: CharVector16SSE2) -> BoolVector16SSE2 {
        unsafe { BoolVector16SSE2(_mm_cmpgt_epi8(self.0, rhs.0)) }
    }

    pub fn m128i(&self) -> __m128i {
        self.0
    }
}

#[cfg(target_feature = "sse2")]
impl std::ops::BitOr<CharVector16SSE2> for CharVector16SSE2 {
    type Output = CharVector16SSE2;

    fn bitor(self, rhs: CharVector16SSE2) -> CharVector16SSE2 {
        unsafe { CharVector16SSE2(_mm_or_si128(self.0, rhs.0)) }
    }
}

#[cfg(target_feature = "sse2")]
#[derive(Debug)]
pub struct BoolVector16SSE2(__m128i);

#[cfg(target_feature = "sse2")]
impl BoolVector16SSE2 {
    // data must point to at least 16 elements.
    pub fn load_slow(data: &[bool]) -> BoolVector16SSE2 {
        qljs_assert!(data.len() >= 16);
        let mut bytes: [u8; 16] = [0; 16]; // TODO(port): Do not initialize.
        for i in 0..16 {
            bytes[i] = if data[i] { 0xff } else { 0x00 };
        }
        unsafe { BoolVector16SSE2(_mm_loadu_si128(bytes.as_ptr() as *const __m128i)) }
    }

    pub fn find_first_false(&self) -> u32 {
        let mask = self.mask();
        if !mask == 0 {
            // HACK(strager): Coerce GCC into omitting a branch due to an if check in
            // countr_one's implementation.
            // TODO(port): Is this hack necessary with LLVM?
            unreachable!();
        }
        mask.trailing_ones()
    }

    pub fn mask(&self) -> u32 {
        unsafe { _mm_movemask_epi8(self.0) as u32 }
    }
}

#[cfg(target_feature = "sse2")]
impl std::ops::BitAnd<BoolVector16SSE2> for BoolVector16SSE2 {
    type Output = BoolVector16SSE2;

    fn bitand(self, rhs: BoolVector16SSE2) -> BoolVector16SSE2 {
        unsafe { BoolVector16SSE2(_mm_and_si128(self.0, rhs.0)) }
    }
}

#[cfg(target_feature = "sse2")]
impl std::ops::BitOr<BoolVector16SSE2> for BoolVector16SSE2 {
    type Output = BoolVector16SSE2;

    fn bitor(self, rhs: BoolVector16SSE2) -> BoolVector16SSE2 {
        unsafe { BoolVector16SSE2(_mm_or_si128(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;

#[cfg(target_feature = "neon")]
pub struct CharVector16NEON(uint8x16_t);

#[cfg(target_feature = "neon")]
impl CharVector16NEON {
    // data must point to at least 16 elements.
    pub fn load(data: &[u8]) -> CharVector16NEON {
        qljs_assert!(data.len() >= 16);
        unsafe { CharVector16NEON(vld1q_u8(data.as_ptr())) }
    }

    pub fn repeated(x: u8) -> CharVector16NEON {
        unsafe { CharVector16NEON(vdupq_n_u8(x)) }
    }

    // out_data must point to at least 16 elements.
    pub fn store(&self, out_data: &mut [u8]) {
        qljs_assert!(out_data.len() >= 16);
        unsafe {
            vst1q_u8(out_data.as_mut_ptr(), self.0);
        }
    }

    pub fn lane_eq(&self, rhs: CharVector16NEON) -> BoolVector16NEON {
        unsafe { BoolVector16NEON(vceqq_u8(self.0, rhs.0)) }
    }

    pub fn lane_lt(&self, rhs: CharVector16NEON) -> BoolVector16NEON {
        unsafe { BoolVector16NEON(vcltq_u8(self.0, rhs.0)) }
    }

    pub fn lane_gt(&self, rhs: CharVector16NEON) -> BoolVector16NEON {
        unsafe { BoolVector16NEON(vcgtq_u8(self.0, rhs.0)) }
    }

    pub fn uint8x16(&self) -> uint8x16_t {
        self.0
    }
}

#[cfg(target_feature = "neon")]
impl std::ops::BitOr<CharVector16NEON> for CharVector16NEON {
    type Output = CharVector16NEON;

    fn bitor(self, rhs: CharVector16NEON) -> CharVector16NEON {
        unsafe { CharVector16NEON(vorrq_u8(self.0, rhs.0)) }
    }
}

#[cfg(target_feature = "neon")]
#[derive(Debug)]
pub struct BoolVector16NEON(pub(crate) uint8x16_t);

#[cfg(target_feature = "neon")]
impl BoolVector16NEON {
    // data must point to at least 16 elements.
    pub fn load_slow(data: &[bool]) -> BoolVector16NEON {
        qljs_assert!(data.len() >= 16);
        let mut bytes: [u8; 16] = [0; 16]; // TODO(port): Do not initialize.
        for i in 0..16 {
            bytes[i] = if data[i] { 0xff } else { 0x00 };
        }
        unsafe { BoolVector16NEON(vld1q_u8(bytes.as_ptr())) }
    }

    // find_first_falls and mask are implemented in simd_neon_arm.rs.
}

#[cfg(target_feature = "neon")]
impl std::ops::BitAnd<BoolVector16NEON> for BoolVector16NEON {
    type Output = BoolVector16NEON;

    fn bitand(self, rhs: BoolVector16NEON) -> BoolVector16NEON {
        unsafe { BoolVector16NEON(vandq_u8(self.0, rhs.0)) }
    }
}

#[cfg(target_feature = "neon")]
impl std::ops::BitOr<BoolVector16NEON> for BoolVector16NEON {
    type Output = BoolVector16NEON;

    fn bitor(self, rhs: BoolVector16NEON) -> BoolVector16NEON {
        unsafe { BoolVector16NEON(vorrq_u8(self.0, rhs.0)) }
    }
}

#[cfg(target_feature = "neon")]
pub use crate::port::simd_neon_arm::*;