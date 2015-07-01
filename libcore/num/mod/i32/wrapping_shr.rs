#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! int_impl {
    //     ($ActualT:ty, $UnsignedT:ty, $BITS:expr,
    //      $add_with_overflow:path,
    //      $sub_with_overflow:path,
    //      $mul_with_overflow:path) => {
    //         /// Returns the smallest value that can be represented by this integer type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn min_value() -> Self {
    //             (-1 as Self) << ($BITS - 1)
    //         }
    //
    //         /// Returns the largest value that can be represented by this integer type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn max_value() -> Self {
    //             let min = Self::min_value(); !min
    //         }
    //
    //         /// Converts a string slice in a given base to an integer.
    //         ///
    //         /// Leading and trailing whitespace represent an error.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```
    //         /// assert_eq!(u32::from_str_radix("A", 16), Ok(10));
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[allow(deprecated)]
    //         pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
    //             from_str_radix(src, radix)
    //         }
    //
    //         /// Returns the number of ones in the binary representation of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b01001100u8;
    //         ///
    //         /// assert_eq!(n.count_ones(), 3);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn count_ones(self) -> u32 { (self as $UnsignedT).count_ones() }
    //
    //         /// Returns the number of zeros in the binary representation of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b01001100u8;
    //         ///
    //         /// assert_eq!(n.count_zeros(), 5);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn count_zeros(self) -> u32 {
    //             (!self).count_ones()
    //         }
    //
    //         /// Returns the number of leading zeros in the binary representation
    //         /// of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b0101000u16;
    //         ///
    //         /// assert_eq!(n.leading_zeros(), 10);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn leading_zeros(self) -> u32 {
    //             (self as $UnsignedT).leading_zeros()
    //         }
    //
    //         /// Returns the number of trailing zeros in the binary representation
    //         /// of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b0101000u16;
    //         ///
    //         /// assert_eq!(n.trailing_zeros(), 3);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn trailing_zeros(self) -> u32 {
    //             (self as $UnsignedT).trailing_zeros()
    //         }
    //
    //         /// Shifts the bits to the left by a specified amount, `n`,
    //         /// wrapping the truncated bits to the end of the resulting integer.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         /// let m = 0x3456789ABCDEF012u64;
    //         ///
    //         /// assert_eq!(n.rotate_left(12), m);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn rotate_left(self, n: u32) -> Self {
    //             (self as $UnsignedT).rotate_left(n) as Self
    //         }
    //
    //         /// Shifts the bits to the right by a specified amount, `n`,
    //         /// wrapping the truncated bits to the beginning of the resulting
    //         /// integer.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         /// let m = 0xDEF0123456789ABCu64;
    //         ///
    //         /// assert_eq!(n.rotate_right(12), m);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn rotate_right(self, n: u32) -> Self {
    //             (self as $UnsignedT).rotate_right(n) as Self
    //         }
    //
    //         /// Reverses the byte order of the integer.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         /// let m = 0xEFCDAB8967452301u64;
    //         ///
    //         /// assert_eq!(n.swap_bytes(), m);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn swap_bytes(self) -> Self {
    //             (self as $UnsignedT).swap_bytes() as Self
    //         }
    //
    //         /// Converts an integer from big endian to the target's endianness.
    //         ///
    //         /// On big endian this is a no-op. On little endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "big") {
    //         ///     assert_eq!(u64::from_be(n), n)
    //         /// } else {
    //         ///     assert_eq!(u64::from_be(n), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn from_be(x: Self) -> Self {
    //             if cfg!(target_endian = "big") { x } else { x.swap_bytes() }
    //         }
    //
    //         /// Converts an integer from little endian to the target's endianness.
    //         ///
    //         /// On little endian this is a no-op. On big endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "little") {
    //         ///     assert_eq!(u64::from_le(n), n)
    //         /// } else {
    //         ///     assert_eq!(u64::from_le(n), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn from_le(x: Self) -> Self {
    //             if cfg!(target_endian = "little") { x } else { x.swap_bytes() }
    //         }
    //
    //         /// Converts `self` to big endian from the target's endianness.
    //         ///
    //         /// On big endian this is a no-op. On little endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "big") {
    //         ///     assert_eq!(n.to_be(), n)
    //         /// } else {
    //         ///     assert_eq!(n.to_be(), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn to_be(self) -> Self { // or not to be?
    //             if cfg!(target_endian = "big") { self } else { self.swap_bytes() }
    //         }
    //
    //         /// Converts `self` to little endian from the target's endianness.
    //         ///
    //         /// On little endian this is a no-op. On big endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "little") {
    //         ///     assert_eq!(n.to_le(), n)
    //         /// } else {
    //         ///     assert_eq!(n.to_le(), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn to_le(self) -> Self {
    //             if cfg!(target_endian = "little") { self } else { self.swap_bytes() }
    //         }
    //
    //         /// Checked integer addition. Computes `self + other`, returning `None`
    //         /// if overflow occurred.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!(5u16.checked_add(65530), Some(65535));
    //         /// assert_eq!(6u16.checked_add(65530), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_add(self, other: Self) -> Option<Self> {
    //             checked_op!($ActualT, $add_with_overflow, self, other)
    //         }
    //
    //         /// Checked integer subtraction. Computes `self - other`, returning
    //         /// `None` if underflow occurred.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!((-127i8).checked_sub(1), Some(-128));
    //         /// assert_eq!((-128i8).checked_sub(1), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_sub(self, other: Self) -> Option<Self> {
    //             checked_op!($ActualT, $sub_with_overflow, self, other)
    //         }
    //
    //         /// Checked integer multiplication. Computes `self * other`, returning
    //         /// `None` if underflow or overflow occurred.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!(5u8.checked_mul(51), Some(255));
    //         /// assert_eq!(5u8.checked_mul(52), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_mul(self, other: Self) -> Option<Self> {
    //             checked_op!($ActualT, $mul_with_overflow, self, other)
    //         }
    //
    //         /// Checked integer division. Computes `self / other`, returning `None`
    //         /// if `other == 0` or the operation results in underflow or overflow.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!((-127i8).checked_div(-1), Some(127));
    //         /// assert_eq!((-128i8).checked_div(-1), None);
    //         /// assert_eq!((1i8).checked_div(0), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_div(self, v: Self) -> Option<Self> {
    //             match v {
    //                 0   => None,
    //                -1 if self == Self::min_value()
    //                     => None,
    //                 v   => Some(self / v),
    //             }
    //         }
    //
    //         /// Saturating integer addition. Computes `self + other`, saturating at
    //         /// the numeric bounds instead of overflowing.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn saturating_add(self, other: Self) -> Self {
    //             match self.checked_add(other) {
    //                 Some(x)                       => x,
    //                 None if other >= Self::zero() => Self::max_value(),
    //                 None => Self::min_value(),
    //             }
    //         }
    //
    //         /// Saturating integer subtraction. Computes `self - other`, saturating
    //         /// at the numeric bounds instead of overflowing.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn saturating_sub(self, other: Self) -> Self {
    //             match self.checked_sub(other) {
    //                 Some(x)                      => x,
    //                 None if other >= Self::zero() => Self::min_value(),
    //                 None => Self::max_value(),
    //             }
    //         }
    //
    //         /// Wrapping (modular) addition. Computes `self + other`,
    //         /// wrapping around at the boundary of the type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn wrapping_add(self, rhs: Self) -> Self {
    //             unsafe {
    //                 intrinsics::overflowing_add(self, rhs)
    //             }
    //         }
    //
    //         /// Wrapping (modular) subtraction. Computes `self - other`,
    //         /// wrapping around at the boundary of the type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn wrapping_sub(self, rhs: Self) -> Self {
    //             unsafe {
    //                 intrinsics::overflowing_sub(self, rhs)
    //             }
    //         }
    //
    //         /// Wrapping (modular) multiplication. Computes `self *
    //         /// other`, wrapping around at the boundary of the type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn wrapping_mul(self, rhs: Self) -> Self {
    //             unsafe {
    //                 intrinsics::overflowing_mul(self, rhs)
    //             }
    //         }
    //
    //         /// Wrapping (modular) division. Computes `floor(self / other)`,
    //         /// wrapping around at the boundary of the type.
    //         ///
    //         /// The only case where such wrapping can occur is when one
    //         /// divides `MIN / -1` on a signed type (where `MIN` is the
    //         /// negative minimal value for the type); this is equivalent
    //         /// to `-MIN`, a positive value that is too large to represent
    //         /// in the type. In such a case, this function returns `MIN`
    //         /// itself..
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_div(self, rhs: Self) -> Self {
    //             self.overflowing_div(rhs).0
    //         }
    //
    //         /// Wrapping (modular) remainder. Computes `self % other`,
    //         /// wrapping around at the boundary of the type.
    //         ///
    //         /// Such wrap-around never actually occurs mathematically;
    //         /// implementation artifacts make `x % y` illegal for `MIN /
    //         /// -1` on a signed type illegal (where `MIN` is the negative
    //         /// minimal value). In such a case, this function returns `0`.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_rem(self, rhs: Self) -> Self {
    //             self.overflowing_rem(rhs).0
    //         }
    //
    //         /// Wrapping (modular) negation. Computes `-self`,
    //         /// wrapping around at the boundary of the type.
    //         ///
    //         /// The only case where such wrapping can occur is when one
    //         /// negates `MIN` on a signed type (where `MIN` is the
    //         /// negative minimal value for the type); this is a positive
    //         /// value that is too large to represent in the type. In such
    //         /// a case, this function returns `MIN` itself.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_neg(self) -> Self {
    //             self.overflowing_neg().0
    //         }
    //
    //         /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    //         /// where `mask` removes any high-order bits of `rhs` that
    //         /// would cause the shift to exceed the bitwidth of the type.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_shl(self, rhs: u32) -> Self {
    //             self.overflowing_shl(rhs).0
    //         }
    //
    //         /// Panic-free bitwise shift-left; yields `self >> mask(rhs)`,
    //         /// where `mask` removes any high-order bits of `rhs` that
    //         /// would cause the shift to exceed the bitwidth of the type.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_shr(self, rhs: u32) -> Self {
    //             self.overflowing_shr(rhs).0
    //         }
    //
    //         /// Raises self to the power of `exp`, using exponentiation by squaring.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```
    //         /// let x: i32 = 2; // or any other integer type
    //         ///
    //         /// assert_eq!(x.pow(4), 16);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn pow(self, mut exp: u32) -> Self {
    //             let mut base = self;
    //             let mut acc = Self::one();
    //
    //             let mut prev_base = self;
    //             let mut base_oflo = false;
    //             while exp > 0 {
    //                 if (exp & 1) == 1 {
    //                     if base_oflo {
    //                         // ensure overflow occurs in the same manner it
    //                         // would have otherwise (i.e. signal any exception
    //                         // it would have otherwise).
    //                         acc = acc * (prev_base * prev_base);
    //                     } else {
    //                         acc = acc * base;
    //                     }
    //                 }
    //                 prev_base = base;
    //                 let (new_base, new_base_oflo) = base.overflowing_mul(base);
    //                 base = new_base;
    //                 base_oflo = new_base_oflo;
    //                 exp /= 2;
    //             }
    //             acc
    //         }
    //
    //         /// Computes the absolute value of `self`.
    //         ///
    //         /// # Overflow behavior
    //         ///
    //         /// The absolute value of `i32::min_value()` cannot be represented as an
    //         /// `i32`, and attempting to calculate it will cause an overflow. This
    //         /// means that code in debug mode will trigger a panic on this case and
    //         /// optimized code will return `i32::min_value()` without a panic.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn abs(self) -> Self {
    //             if self.is_negative() {
    //                 // Note that the #[inline] above means that the overflow
    //                 // semantics of this negation depend on the crate we're being
    //                 // inlined into.
    //                 -self
    //             } else {
    //                 self
    //             }
    //         }
    //
    //         /// Returns a number representing sign of `self`.
    //         ///
    //         /// - `0` if the number is zero
    //         /// - `1` if the number is positive
    //         /// - `-1` if the number is negative
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn signum(self) -> Self {
    //             match self {
    //                 n if n > 0 =>  1,
    //                 0          =>  0,
    //                 _          => -1,
    //             }
    //         }
    //
    //         /// Returns `true` if `self` is positive and `false` if the number
    //         /// is zero or negative.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn is_positive(self) -> bool { self > 0 }
    //
    //         /// Returns `true` if `self` is negative and `false` if the number
    //         /// is zero or positive.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn is_negative(self) -> bool { self < 0 }
    //     }
    // }

    // impl i32 {
    //     int_impl! { i32, u32, 32,
    //         intrinsics::i32_add_with_overflow,
    //         intrinsics::i32_sub_with_overflow,
    //         intrinsics::i32_mul_with_overflow }
    // }

    macro_rules! wrapping_shr_test {
	($value:expr, $rhs:expr, $result:expr) => ({
	    let x: i32 = $value;
	    let rhs: u32  = $rhs;
	    let result: i32 = x.wrapping_shr(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn wrapping_shr_test1() {
	wrapping_shr_test!( 0x80000000, 0, 0x80000000 );
	wrapping_shr_test!( 0x80000000, 1, 0xc0000000 );
	wrapping_shr_test!( 0x80000000, 2, 0xe0000000 );
	wrapping_shr_test!( 0x80000000, 3, 0xf0000000 );
	wrapping_shr_test!( 0x80000000, 4, 0xf8000000 );
	wrapping_shr_test!( 0x80000000, 5, 0xfc000000 );
	wrapping_shr_test!( 0x80000000, 6, 0xfe000000 );
	wrapping_shr_test!( 0x80000000, 7, 0xff000000 );
	wrapping_shr_test!( 0x80000000, 8, 0xff800000 );
	wrapping_shr_test!( 0x80000000, 9, 0xffc00000 );
	wrapping_shr_test!( 0x80000000, 10, 0xffe00000 );
	wrapping_shr_test!( 0x80000000, 11, 0xfff00000 );
	wrapping_shr_test!( 0x80000000, 12, 0xfff80000 );
	wrapping_shr_test!( 0x80000000, 13, 0xfffc0000 );
	wrapping_shr_test!( 0x80000000, 14, 0xfffe0000 );
	wrapping_shr_test!( 0x80000000, 15, 0xffff0000 );
	wrapping_shr_test!( 0x80000000, 16, 0xffff8000 );
	wrapping_shr_test!( 0x80000000, 17, 0xffffc000 );
	wrapping_shr_test!( 0x80000000, 18, 0xffffe000 );
	wrapping_shr_test!( 0x80000000, 19, 0xfffff000 );
	wrapping_shr_test!( 0x80000000, 20, 0xfffff800 );
	wrapping_shr_test!( 0x80000000, 21, 0xfffffc00 );
	wrapping_shr_test!( 0x80000000, 22, 0xfffffe00 );
	wrapping_shr_test!( 0x80000000, 23, 0xffffff00 );
	wrapping_shr_test!( 0x80000000, 24, 0xffffff80 );
	wrapping_shr_test!( 0x80000000, 25, 0xffffffc0 );
	wrapping_shr_test!( 0x80000000, 26, 0xffffffe0 );
	wrapping_shr_test!( 0x80000000, 27, 0xfffffff0 );
	wrapping_shr_test!( 0x80000000, 28, 0xfffffff8 );
	wrapping_shr_test!( 0x80000000, 29, 0xfffffffc );
	wrapping_shr_test!( 0x80000000, 30, 0xfffffffe );
	wrapping_shr_test!( 0x80000000, 31, 0xffffffff );
	wrapping_shr_test!( 0x80000000, 32, 0x80000000 );
    }
}
