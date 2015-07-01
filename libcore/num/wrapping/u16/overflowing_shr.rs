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
	overflowing_shr_test!( u16, 0x8000, 0, (0x8000, false) );
	overflowing_shr_test!( u16, 0x8000, 1, (0x4000, false) );
	overflowing_shr_test!( u16, 0x8000, 2, (0x2000, false) );
	overflowing_shr_test!( u16, 0x8000, 3, (0x1000, false) );
	overflowing_shr_test!( u16, 0x8000, 4, (0x0800, false) );
	overflowing_shr_test!( u16, 0x8000, 5, (0x0400, false) );
	overflowing_shr_test!( u16, 0x8000, 6, (0x0200, false) );
	overflowing_shr_test!( u16, 0x8000, 7, (0x0100, false) );
	overflowing_shr_test!( u16, 0x8000, 8, (0x0080, false) );
	overflowing_shr_test!( u16, 0x8000, 9, (0x0040, false) );
	overflowing_shr_test!( u16, 0x8000, 10, (0x0020, false) );
	overflowing_shr_test!( u16, 0x8000, 11, (0x0010, false) );
	overflowing_shr_test!( u16, 0x8000, 12, (0x0008, false) );
	overflowing_shr_test!( u16, 0x8000, 13, (0x0004, false) );
	overflowing_shr_test!( u16, 0x8000, 14, (0x0002, false) );
	overflowing_shr_test!( u16, 0x8000, 15, (0x0001, false) );
    }

    #[test]
    #[should_panic]
    fn overflowing_shr_test2() {
	overflowing_shr_test!( u16, 0x8000, 16, (0x0000, true) );
    }
}
