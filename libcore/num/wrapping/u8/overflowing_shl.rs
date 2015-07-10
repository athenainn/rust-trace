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
	overflowing_shl_test!( u8, 0x01, 0, (0x01, false) );
	overflowing_shl_test!( u8, 0x01, 1, (0x02, false) );
	overflowing_shl_test!( u8, 0x01, 2, (0x04, false) );
	overflowing_shl_test!( u8, 0x01, 3, (0x08, false) );
	overflowing_shl_test!( u8, 0x01, 4, (0x10, false) );
	overflowing_shl_test!( u8, 0x01, 5, (0x20, false) );
	overflowing_shl_test!( u8, 0x01, 6, (0x40, false) );
	overflowing_shl_test!( u8, 0x01, 7, (0x80, false) );
    }

    #[test]
    #[should_panic]
    fn overflowing_shl_test2() {
	overflowing_shl_test!( u8, 0x01, 8, (0x00, true) );
    }
}
