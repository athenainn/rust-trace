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
	overflowing_shr_test!( u32, 0x80000000, 0, (0x80000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 1, (0x40000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 2, (0x20000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 3, (0x10000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 4, (0x08000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 5, (0x04000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 6, (0x02000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 7, (0x01000000, false) );
	overflowing_shr_test!( u32, 0x80000000, 8, (0x00800000, false) );
	overflowing_shr_test!( u32, 0x80000000, 9, (0x00400000, false) );
	overflowing_shr_test!( u32, 0x80000000, 10, (0x00200000, false) );
	overflowing_shr_test!( u32, 0x80000000, 11, (0x00100000, false) );
	overflowing_shr_test!( u32, 0x80000000, 12, (0x00080000, false) );
	overflowing_shr_test!( u32, 0x80000000, 13, (0x00040000, false) );
	overflowing_shr_test!( u32, 0x80000000, 14, (0x00020000, false) );
	overflowing_shr_test!( u32, 0x80000000, 15, (0x00010000, false) );
	overflowing_shr_test!( u32, 0x80000000, 16, (0x00008000, false) );
	overflowing_shr_test!( u32, 0x80000000, 17, (0x00004000, false) );
	overflowing_shr_test!( u32, 0x80000000, 18, (0x00002000, false) );
	overflowing_shr_test!( u32, 0x80000000, 19, (0x00001000, false) );
	overflowing_shr_test!( u32, 0x80000000, 20, (0x00000800, false) );
	overflowing_shr_test!( u32, 0x80000000, 21, (0x00000400, false) );
	overflowing_shr_test!( u32, 0x80000000, 22, (0x00000200, false) );
	overflowing_shr_test!( u32, 0x80000000, 23, (0x00000100, false) );
	overflowing_shr_test!( u32, 0x80000000, 24, (0x00000080, false) );
	overflowing_shr_test!( u32, 0x80000000, 25, (0x00000040, false) );
	overflowing_shr_test!( u32, 0x80000000, 26, (0x00000020, false) );
	overflowing_shr_test!( u32, 0x80000000, 27, (0x00000010, false) );
	overflowing_shr_test!( u32, 0x80000000, 28, (0x00000008, false) );
	overflowing_shr_test!( u32, 0x80000000, 29, (0x00000004, false) );
	overflowing_shr_test!( u32, 0x80000000, 30, (0x00000002, false) );
	overflowing_shr_test!( u32, 0x80000000, 31, (0x00000001, false) );
    }

    #[test]
    #[should_panic]
    fn overflowing_shr_test2() {
	overflowing_shr_test!( u32, 0x80000000, 32, (0x00000000, true) );
    }
}
