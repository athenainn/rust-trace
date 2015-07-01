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

    // macro_rules! unsigned_overflowing_impl {
    //     ($($t:ident)*) => ($(
    //         impl OverflowingOps for $t {
    //             #[inline(always)]
    //             fn overflowing_add(self, rhs: $t) -> ($t, bool) {
    //                 unsafe {
    //                     concat_idents!($t, _add_with_overflow)(self, rhs)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn overflowing_sub(self, rhs: $t) -> ($t, bool) {
    //                 unsafe {
    //                     concat_idents!($t, _sub_with_overflow)(self, rhs)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn overflowing_mul(self, rhs: $t) -> ($t, bool) {
    //                 unsafe {
    //                     concat_idents!($t, _mul_with_overflow)(self, rhs)
    //                 }
    //             }
    //
    //             #[inline(always)]
    //             fn overflowing_div(self, rhs: $t) -> ($t, bool) {
    //                 (self/rhs, false)
    //             }
    //             #[inline(always)]
    //             fn overflowing_rem(self, rhs: $t) -> ($t, bool) {
    //                 (self % rhs, false)
    //             }
    //
    //             #[inline(always)]
    //             fn overflowing_shl(self, rhs: u32) -> ($t, bool) {
    //                 (self << (rhs & self::shift_max::$t),
    //                  (rhs > self::shift_max::$t))
    //             }
    //             #[inline(always)]
    //             fn overflowing_shr(self, rhs: u32) -> ($t, bool) {
    //                 (self >> (rhs & self::shift_max::$t),
    //                  (rhs > self::shift_max::$t))
    //             }
    //
    //             #[inline(always)]
    //             fn overflowing_neg(self) -> ($t, bool) {
    //                 ((!self).wrapping_add(1), true)
    //             }
    //         }
    //     )*)
    // }

    // unsigned_overflowing_impl! { u8 u16 u32 u64 }

    macro_rules! overflowing_shr_test {
	($T:ty, $value:expr, $rhs:expr, $result:expr) => ({
	    let value: $T = $value;
	    let rhs: u32 = $rhs;
	    let result: ($T, bool) = value.overflowing_shr(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn overflowing_shr_test1() {
	overflowing_shr_test!( u64, 0x8000000000000000, 0, (0x8000000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 1, (0x4000000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 2, (0x2000000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 3, (0x1000000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 4, (0x0800000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 5, (0x0400000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 6, (0x0200000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 7, (0x0100000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 8, (0x0080000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 9, (0x0040000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 10, (0x0020000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 11, (0x0010000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 12, (0x0008000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 13, (0x0004000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 14, (0x0002000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 15, (0x0001000000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 16, (0x0000800000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 17, (0x0000400000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 18, (0x0000200000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 19, (0x0000100000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 20, (0x0000080000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 21, (0x0000040000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 22, (0x0000020000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 23, (0x0000010000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 24, (0x0000008000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 25, (0x0000004000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 26, (0x0000002000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 27, (0x0000001000000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 28, (0x0000000800000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 29, (0x0000000400000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 30, (0x0000000200000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 31, (0x0000000100000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 32, (0x0000000080000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 33, (0x0000000040000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 34, (0x0000000020000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 35, (0x0000000010000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 36, (0x0000000008000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 37, (0x0000000004000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 38, (0x0000000002000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 39, (0x0000000001000000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 40, (0x0000000000800000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 41, (0x0000000000400000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 42, (0x0000000000200000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 43, (0x0000000000100000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 44, (0x0000000000080000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 45, (0x0000000000040000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 46, (0x0000000000020000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 47, (0x0000000000010000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 48, (0x0000000000008000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 49, (0x0000000000004000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 50, (0x0000000000002000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 51, (0x0000000000001000, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 52, (0x0000000000000800, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 53, (0x0000000000000400, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 54, (0x0000000000000200, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 55, (0x0000000000000100, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 56, (0x0000000000000080, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 57, (0x0000000000000040, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 58, (0x0000000000000020, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 59, (0x0000000000000010, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 60, (0x0000000000000008, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 61, (0x0000000000000004, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 62, (0x0000000000000002, false) );
	overflowing_shr_test!( u64, 0x8000000000000000, 63, (0x0000000000000001, false) );
    }

    #[test]
    #[should_panic]
    fn overflowing_shr_test2() {
	overflowing_shr_test!( u64, 0x8000000000000000, 64, (0x0000000000000000, true) );
    }
}
