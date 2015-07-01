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
	overflowing_shr_test!( i64, 0x8000000000000000, 0, (0x8000000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 1, (0xc000000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 2, (0xe000000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 3, (0xf000000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 4, (0xf800000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 5, (0xfc00000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 6, (0xfe00000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 7, (0xff00000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 8, (0xff80000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 9, (0xffc0000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 10, (0xffe0000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 11, (0xfff0000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 12, (0xfff8000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 13, (0xfffc000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 14, (0xfffe000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 15, (0xffff000000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 16, (0xffff800000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 17, (0xffffc00000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 18, (0xffffe00000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 19, (0xfffff00000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 20, (0xfffff80000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 21, (0xfffffc0000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 22, (0xfffffe0000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 23, (0xffffff0000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 24, (0xffffff8000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 25, (0xffffffc000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 26, (0xffffffe000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 27, (0xfffffff000000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 28, (0xfffffff800000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 29, (0xfffffffc00000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 30, (0xfffffffe00000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 31, (0xffffffff00000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 32, (0xffffffff80000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 33, (0xffffffffc0000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 34, (0xffffffffe0000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 35, (0xfffffffff0000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 36, (0xfffffffff8000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 37, (0xfffffffffc000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 38, (0xfffffffffe000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 39, (0xffffffffff000000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 40, (0xffffffffff800000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 41, (0xffffffffffc00000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 42, (0xffffffffffe00000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 43, (0xfffffffffff00000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 44, (0xfffffffffff80000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 45, (0xfffffffffffc0000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 46, (0xfffffffffffe0000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 47, (0xffffffffffff0000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 48, (0xffffffffffff8000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 49, (0xffffffffffffc000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 50, (0xffffffffffffe000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 51, (0xfffffffffffff000, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 52, (0xfffffffffffff800, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 53, (0xfffffffffffffc00, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 54, (0xfffffffffffffe00, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 55, (0xffffffffffffff00, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 56, (0xffffffffffffff80, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 57, (0xffffffffffffffc0, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 58, (0xffffffffffffffe0, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 59, (0xfffffffffffffff0, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 60, (0xfffffffffffffff8, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 61, (0xfffffffffffffffc, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 62, (0xfffffffffffffffe, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 63, (0xffffffffffffffff, false) );
	overflowing_shr_test!( i64, 0x8000000000000000, 64, (0x8000000000000000, true) );
    }
}
