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

    // macro_rules! signed_overflowing_impl {
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
    //                 if self == $t::MIN && rhs == -1 {
    //                     (self, true)
    //                 } else {
    //                     (self/rhs, false)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn overflowing_rem(self, rhs: $t) -> ($t, bool) {
    //                 if self == $t::MIN && rhs == -1 {
    //                     (0, true)
    //                 } else {
    //                     (self % rhs, false)
    //                 }
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
    //                 if self == $t::MIN {
    //                     ($t::MIN, true)
    //                 } else {
    //                     (-self, false)
    //                 }
    //             }
    //         }
    //     )*)
    // }

    // signed_overflowing_impl! { i8 i16 i32 i64 }

    macro_rules! overflowing_shr_test {
	($T:ty, $value:expr, $rhs:expr, $result:expr) => ({
	    let value: $T = $value;
	    let rhs: u32 = $rhs;
	    let result: ($T, bool) = value.overflowing_shr(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn overflowing_shr_test1() {
	overflowing_shr_test!( i32, 0x80000000, 0, (0x80000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 1, (0xc0000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 2, (0xe0000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 3, (0xf0000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 4, (0xf8000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 5, (0xfc000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 6, (0xfe000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 7, (0xff000000, false) );
	overflowing_shr_test!( i32, 0x80000000, 8, (0xff800000, false) );
	overflowing_shr_test!( i32, 0x80000000, 9, (0xffc00000, false) );
	overflowing_shr_test!( i32, 0x80000000, 10, (0xffe00000, false) );
	overflowing_shr_test!( i32, 0x80000000, 11, (0xfff00000, false) );
	overflowing_shr_test!( i32, 0x80000000, 12, (0xfff80000, false) );
	overflowing_shr_test!( i32, 0x80000000, 13, (0xfffc0000, false) );
	overflowing_shr_test!( i32, 0x80000000, 14, (0xfffe0000, false) );
	overflowing_shr_test!( i32, 0x80000000, 15, (0xffff0000, false) );
	overflowing_shr_test!( i32, 0x80000000, 16, (0xffff8000, false) );
	overflowing_shr_test!( i32, 0x80000000, 17, (0xffffc000, false) );
	overflowing_shr_test!( i32, 0x80000000, 18, (0xffffe000, false) );
	overflowing_shr_test!( i32, 0x80000000, 19, (0xfffff000, false) );
	overflowing_shr_test!( i32, 0x80000000, 20, (0xfffff800, false) );
	overflowing_shr_test!( i32, 0x80000000, 21, (0xfffffc00, false) );
	overflowing_shr_test!( i32, 0x80000000, 22, (0xfffffe00, false) );
	overflowing_shr_test!( i32, 0x80000000, 23, (0xffffff00, false) );
	overflowing_shr_test!( i32, 0x80000000, 24, (0xffffff80, false) );
	overflowing_shr_test!( i32, 0x80000000, 25, (0xffffffc0, false) );
	overflowing_shr_test!( i32, 0x80000000, 26, (0xffffffe0, false) );
	overflowing_shr_test!( i32, 0x80000000, 27, (0xfffffff0, false) );
	overflowing_shr_test!( i32, 0x80000000, 28, (0xfffffff8, false) );
	overflowing_shr_test!( i32, 0x80000000, 29, (0xfffffffc, false) );
	overflowing_shr_test!( i32, 0x80000000, 30, (0xfffffffe, false) );
	overflowing_shr_test!( i32, 0x80000000, 31, (0xffffffff, false) );
	overflowing_shr_test!( i32, 0x80000000, 32, (0x80000000, true) );
    }
}
