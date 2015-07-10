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
    // impl OverflowingOps for usize {
    //     #[inline(always)]
    //     fn overflowing_add(self, rhs: usize) -> (usize, bool) {
    //         unsafe {
    //             let res = u64_add_with_overflow(self as u64, rhs as u64);
    //             (res.0 as usize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_sub(self, rhs: usize) -> (usize, bool) {
    //         unsafe {
    //             let res = u64_sub_with_overflow(self as u64, rhs as u64);
    //             (res.0 as usize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_mul(self, rhs: usize) -> (usize, bool) {
    //         unsafe {
    //             let res = u64_mul_with_overflow(self as u64, rhs as u64);
    //             (res.0 as usize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_div(self, rhs: usize) -> (usize, bool) {
    //         let (r, f) = (self as u64).overflowing_div(rhs as u64);
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_rem(self, rhs: usize) -> (usize, bool) {
    //         let (r, f) = (self as u64).overflowing_rem(rhs as u64);
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_neg(self) -> (usize, bool) {
    //         let (r, f) = (self as u64).overflowing_neg();
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shl(self, rhs: u32) -> (usize, bool) {
    //         let (r, f) = (self as u64).overflowing_shl(rhs);
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shr(self, rhs: u32) -> (usize, bool) {
    //         let (r, f) = (self as u64).overflowing_shr(rhs);
    //         (r as usize, f)
    //     }
    // }
    //
    // #[cfg(target_pointer_width = "32")]
    // impl OverflowingOps for usize {
    //     #[inline(always)]
    //     fn overflowing_add(self, rhs: usize) -> (usize, bool) {
    //         unsafe {
    //             let res = u32_add_with_overflow(self as u32, rhs as u32);
    //             (res.0 as usize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_sub(self, rhs: usize) -> (usize, bool) {
    //         unsafe {
    //             let res = u32_sub_with_overflow(self as u32, rhs as u32);
    //             (res.0 as usize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_mul(self, rhs: usize) -> (usize, bool) {
    //         unsafe {
    //             let res = u32_mul_with_overflow(self as u32, rhs as u32);
    //             (res.0 as usize, res.1)
    //         }
    //     }
    //     #[inline(always)]
    //     fn overflowing_div(self, rhs: usize) -> (usize, bool) {
    //         let (r, f) = (self as u32).overflowing_div(rhs as u32);
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_rem(self, rhs: usize) -> (usize, bool) {
    //         let (r, f) = (self as u32).overflowing_rem(rhs as u32);
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_neg(self) -> (usize, bool) {
    //         let (r, f) = (self as u32).overflowing_neg();
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shl(self, rhs: u32) -> (usize, bool) {
    //         let (r, f) = (self as u32).overflowing_shl(rhs);
    //         (r as usize, f)
    //     }
    //     #[inline(always)]
    //     fn overflowing_shr(self, rhs: u32) -> (usize, bool) {
    //         let (r, f) = (self as u32).overflowing_shr(rhs);
    //         (r as usize, f)
    //     }
    // }

    macro_rules! overflowing_shl_test {
	($T:ty, $value:expr, $rhs:expr, $result:expr) => ({
	    let value: $T = $value;
	    let rhs: u32 = $rhs;
	    let result: ($T, bool) = value.overflowing_shl(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn overflowing_shl_test1() {
	if cfg!(target_pointer_width = "64") {
	    overflowing_shl_test!( usize, 0x0000000000000001, 0, (0x0000000000000001, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 1, (0x0000000000000002, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 2, (0x0000000000000004, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 3, (0x0000000000000008, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 4, (0x0000000000000010, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 5, (0x0000000000000020, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 6, (0x0000000000000040, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 7, (0x0000000000000080, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 8, (0x0000000000000100, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 9, (0x0000000000000200, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 10, (0x0000000000000400, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 11, (0x0000000000000800, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 12, (0x0000000000001000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 13, (0x0000000000002000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 14, (0x0000000000004000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 15, (0x0000000000008000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 16, (0x0000000000010000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 17, (0x0000000000020000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 18, (0x0000000000040000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 19, (0x0000000000080000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 20, (0x0000000000100000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 21, (0x0000000000200000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 22, (0x0000000000400000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 23, (0x0000000000800000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 24, (0x0000000001000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 25, (0x0000000002000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 26, (0x0000000004000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 27, (0x0000000008000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 28, (0x0000000010000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 29, (0x0000000020000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 30, (0x0000000040000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 31, (0x0000000080000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 32, (0x0000000100000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 33, (0x0000000200000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 34, (0x0000000400000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 35, (0x0000000800000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 36, (0x0000001000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 37, (0x0000002000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 38, (0x0000004000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 39, (0x0000008000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 40, (0x0000010000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 41, (0x0000020000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 42, (0x0000040000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 43, (0x0000080000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 44, (0x0000100000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 45, (0x0000200000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 46, (0x0000400000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 47, (0x0000800000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 48, (0x0001000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 49, (0x0002000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 50, (0x0004000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 51, (0x0008000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 52, (0x0010000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 53, (0x0020000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 54, (0x0040000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 55, (0x0080000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 56, (0x0100000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 57, (0x0200000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 58, (0x0400000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 59, (0x0800000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 60, (0x1000000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 61, (0x2000000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 62, (0x4000000000000000, false) );
	    overflowing_shl_test!( usize, 0x0000000000000001, 63, (0x8000000000000000, false) );
	} else {
	    overflowing_shl_test!( usize, 0x00000001, 0, (0x00000001, false) );
	    overflowing_shl_test!( usize, 0x00000001, 1, (0x00000002, false) );
	    overflowing_shl_test!( usize, 0x00000001, 2, (0x00000004, false) );
	    overflowing_shl_test!( usize, 0x00000001, 3, (0x00000008, false) );
	    overflowing_shl_test!( usize, 0x00000001, 4, (0x00000010, false) );
	    overflowing_shl_test!( usize, 0x00000001, 5, (0x00000020, false) );
	    overflowing_shl_test!( usize, 0x00000001, 6, (0x00000040, false) );
	    overflowing_shl_test!( usize, 0x00000001, 7, (0x00000080, false) );
	    overflowing_shl_test!( usize, 0x00000001, 8, (0x00000100, false) );
	    overflowing_shl_test!( usize, 0x00000001, 9, (0x00000200, false) );
	    overflowing_shl_test!( usize, 0x00000001, 10, (0x00000400, false) );
	    overflowing_shl_test!( usize, 0x00000001, 11, (0x00000800, false) );
	    overflowing_shl_test!( usize, 0x00000001, 12, (0x00001000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 13, (0x00002000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 14, (0x00004000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 15, (0x00008000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 16, (0x00010000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 17, (0x00020000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 18, (0x00040000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 19, (0x00080000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 20, (0x00100000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 21, (0x00200000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 22, (0x00400000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 23, (0x00800000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 24, (0x01000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 25, (0x02000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 26, (0x04000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 27, (0x08000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 28, (0x10000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 29, (0x20000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 30, (0x40000000, false) );
	    overflowing_shl_test!( usize, 0x00000001, 31, (0x80000000, false) );
	}
    }

    #[test]
    #[should_panic]
    fn overflowing_shl_test2() {
	if cfg!(target_pointer_width = "64") {
	    overflowing_shl_test!( usize, 0x0000000000000001, 64, (0x0000000000000000, true) );
	} else {
	    overflowing_shl_test!( usize, 0x00000001, 32, (0x00000000, true) );
	}
    }
}
