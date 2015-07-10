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

    macro_rules! overflowing_div_test {
	($T:ty, $value:expr, $rhs:expr, $result:expr) => ({
	    let value: $T = $value;
	    let rhs: $T = $rhs;
	    let result: ($T, bool) = value.overflowing_div(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn overflowing_div_test1() {
	let always_false: bool = false;

	if cfg!(target_pointer_width = "64") {
	    overflowing_div_test!( usize, 0xffffffffffffffff, 0x0000000000000001, (0xffffffffffffffff, always_false) );
	    overflowing_div_test!( usize, 0xffffffffffffffff, 0x0000000000000002, (0x7fffffffffffffff, always_false) );
	} else {
	    overflowing_div_test!( usize, 0xffffffff, 0x00000001, (0xffffffff, false) );
	    overflowing_div_test!( usize, 0xffffffff, 0x00000002, (0x7fffffff, true) );
	}
    }

    #[test]
    #[should_panic]
    fn overflowing_div_test2() {
	let always_false: bool = false;

	if cfg!(target_pointer_width = "64") {
	    overflowing_div_test!( usize, 0xffffffffffffffff, 0x0000000000000000, (0xffffffffffffffff, always_false) ); // panicked at 'attempted to divide by zero'
	} else {
	    overflowing_div_test!( usize, 0xffffffff, 0x00000000, (0xffffffff, always_false) ); // panicked at 'attempted to divide by zero'
	}
    }
}
