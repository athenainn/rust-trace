#![feature(core, wrapping)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::wrapping::OverflowingOps;

    // mod shift_max {
    //     #![allow(non_upper_case_globals)]
    //
    //     pub const  i8: u32 = (1 << 3) - 1;
    //     pub const i16: u32 = (1 << 4) - 1;
    //     pub const i32: u32 = (1 << 5) - 1;
    //     pub const i64: u32 = (1 << 6) - 1;
    //
    //     pub const  u8: u32 = i8;
    //     pub const u16: u32 = i16;
    //     pub const u32: u32 = i32;
    //     pub const u64: u32 = i64;
    // }

    // #[cfg(target_pointer_width = "64")]
    // impl OverflowingOps for isize {
    //     #[inline(always)]
    //     fn overflowing_add(self, rhs: isize) -> (isize, bool) {
    //         unsafe {
    //             let res = i64_add_with_overflow(self as i64, rhs as i64);
    //             (res.0 as isize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_sub(self, rhs: isize) -> (isize, bool) {
    //         unsafe {
    //             let res = i64_sub_with_overflow(self as i64, rhs as i64);
    //             (res.0 as isize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_mul(self, rhs: isize) -> (isize, bool) {
    //         unsafe {
    //             let res = i64_mul_with_overflow(self as i64, rhs as i64);
    //             (res.0 as isize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_div(self, rhs: isize) -> (isize, bool) {
    //         let (r, f) = (self as i64).overflowing_div(rhs as i64);
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_rem(self, rhs: isize) -> (isize, bool) {
    //         let (r, f) = (self as i64).overflowing_rem(rhs as i64);
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_neg(self) -> (isize, bool) {
    //         let (r, f) = (self as i64).overflowing_neg();
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shl(self, rhs: u32) -> (isize, bool) {
    //         let (r, f) = (self as i64).overflowing_shl(rhs);
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shr(self, rhs: u32) -> (isize, bool) {
    //         let (r, f) = (self as i64).overflowing_shr(rhs);
    //         (r as isize, f)
    //     }
    // }
    //
    // #[cfg(target_pointer_width = "32")]
    // impl OverflowingOps for isize {
    //     #[inline(always)]
    //     fn overflowing_add(self, rhs: isize) -> (isize, bool) {
    //         unsafe {
    //             let res = i32_add_with_overflow(self as i32, rhs as i32);
    //             (res.0 as isize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_sub(self, rhs: isize) -> (isize, bool) {
    //         unsafe {
    //             let res = i32_sub_with_overflow(self as i32, rhs as i32);
    //             (res.0 as isize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_mul(self, rhs: isize) -> (isize, bool) {
    //         unsafe {
    //             let res = i32_mul_with_overflow(self as i32, rhs as i32);
    //             (res.0 as isize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_div(self, rhs: isize) -> (isize, bool) {
    //         let (r, f) = (self as i32).overflowing_div(rhs as i32);
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_rem(self, rhs: isize) -> (isize, bool) {
    //         let (r, f) = (self as i32).overflowing_rem(rhs as i32);
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_neg(self) -> (isize, bool) {
    //         let (r, f) = (self as i32).overflowing_neg();
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shl(self, rhs: u32) -> (isize, bool) {
    //         let (r, f) = (self as i32).overflowing_shl(rhs);
    //         (r as isize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shr(self, rhs: u32) -> (isize, bool) {
    //         let (r, f) = (self as i32).overflowing_shr(rhs);
    //         (r as isize, f)
    //     }
    // }

    macro_rules! overflowing_neg_test {
	($T:ty, $value:expr, $result:expr) => ({
	    let value: $T = $value;
	    let result: ($T, bool) = value.overflowing_neg();

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn overflowing_neg_test1() {
	if cfg!(target_pointer_width = "64") {
	    overflowing_neg_test!( isize, 0x7fffffffffffffff, (0x8000000000000001, false) );
	    overflowing_neg_test!( isize, 0x8000000000000000, (0x8000000000000000, true) );
	} else {
	    overflowing_neg_test!( isize, 0x7fffffff, (0x80000001, false) );
	    overflowing_neg_test!( isize, 0x80000000, (0x80000000, true) );
	}
    }
}
