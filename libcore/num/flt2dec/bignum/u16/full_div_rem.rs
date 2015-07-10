#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::bignum::FullOps;

    // pub trait FullOps {
    //     /// Returns `(carry', v')` such that `carry' * 2^W + v' = self + other + carry`,
    //     /// where `W` is the number of bits in `Self`.
    //     fn full_add(self, other: Self, carry: bool) -> (bool /*carry*/, Self);
    //
    //     /// Returns `(carry', v')` such that `carry' * 2^W + v' = self * other + carry`,
    //     /// where `W` is the number of bits in `Self`.
    //     fn full_mul(self, other: Self, carry: Self) -> (Self /*carry*/, Self);
    //
    //     /// Returns `(carry', v')` such that `carry' * 2^W + v' = self * other + other2 + carry`,
    //     /// where `W` is the number of bits in `Self`.
    //     fn full_mul_add(self, other: Self, other2: Self, carry: Self) -> (Self /*carry*/, Self);
    //
    //     /// Returns `(quo, rem)` such that `borrow * 2^W + self = quo * other + rem`
    //     /// and `0 <= rem < other`, where `W` is the number of bits in `Self`.
    //     fn full_div_rem(self, other: Self, borrow: Self) -> (Self /*quotient*/, Self /*remainder*/);
    // }

    // macro_rules! impl_full_ops {
    //     ($($ty:ty: add($addfn:path), mul/div($bigty:ident);)*) => (
    //         $(
    //             impl FullOps for $ty {
    //                 fn full_add(self, other: $ty, carry: bool) -> (bool, $ty) {
    //                     // this cannot overflow, the output is between 0 and 2*2^nbits - 1
    //                     // FIXME will LLVM optimize this into ADC or similar???
    //                     let (v, carry1) = unsafe { $addfn(self, other) };
    //                     let (v, carry2) = unsafe { $addfn(v, if carry {1} else {0}) };
    //                     (carry1 || carry2, v)
    //                 }
    //
    //                 fn full_mul(self, other: $ty, carry: $ty) -> ($ty, $ty) {
    //                     // this cannot overflow, the output is between 0 and 2^nbits * (2^nbits - 1)
    //                     let nbits = mem::size_of::<$ty>() * 8;
    //                     let v = (self as $bigty) * (other as $bigty) + (carry as $bigty);
    //                     ((v >> nbits) as $ty, v as $ty)
    //                 }
    //
    //                 fn full_mul_add(self, other: $ty, other2: $ty, carry: $ty) -> ($ty, $ty) {
    //                     // this cannot overflow, the output is between 0 and 2^(2*nbits) - 1
    //                     let nbits = mem::size_of::<$ty>() * 8;
    //                     let v = (self as $bigty) * (other as $bigty) + (other2 as $bigty) +
    //                             (carry as $bigty);
    //                     ((v >> nbits) as $ty, v as $ty)
    //                 }
    //
    //                 fn full_div_rem(self, other: $ty, borrow: $ty) -> ($ty, $ty) {
    //                     debug_assert!(borrow < other);
    //                     // this cannot overflow, the dividend is between 0 and other * 2^nbits - 1
    //                     let nbits = mem::size_of::<$ty>() * 8;
    //                     let lhs = ((borrow as $bigty) << nbits) | (self as $bigty);
    //                     let rhs = other as $bigty;
    //                     ((lhs / rhs) as $ty, (lhs % rhs) as $ty)
    //                 }
    //             }
    //         )*
    //     )
    // }
    //
    // impl_full_ops! {
    //     u8:  add(intrinsics::u8_add_with_overflow),  mul/div(u16);
    //     u16: add(intrinsics::u16_add_with_overflow), mul/div(u32);
    //     u32: add(intrinsics::u32_add_with_overflow), mul/div(u64);
    // //  u64: add(intrinsics::u64_add_with_overflow), mul/div(u128); // see RFC #521 for enabling this.
    // }

    #[test]
    fn full_div_rem_test1() {
	let value: u16 = 68;
	let other: u16 = 10;
	let borrow: u16 = 0;

	debug_assert!(borrow < other);
	let (quotient, remain): (u16, u16) = value.full_div_rem(other, borrow);

	assert_eq!(quotient, 68 / 10);
	assert_eq!(remain, 68 % 10);
    }

    #[test]
    fn full_div_rem_test2() {
	let value: u16 = 68;
	let other: u16 = 10;
	let borrow: u16 = 5;

	debug_assert!(borrow < other);
	let (quotient, remain): (u16, u16) = value.full_div_rem(other, borrow);

	assert_eq!(quotient, ((5u32 * 0x000010000u32 + 68u32) / 10u32) as u16);
	assert_eq!(remain, 8);
    }
}
