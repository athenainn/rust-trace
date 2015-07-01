#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! uint_impl {
    //     ($ActualT:ty, $BITS:expr,
    //      $ctpop:path,
    //      $ctlz:path,
    //      $cttz:path,
    //      $bswap:path,
    //      $add_with_overflow:path,
    //      $sub_with_overflow:path,
    //      $mul_with_overflow:path) => {
    //         /// Returns the smallest value that can be represented by this integer type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         pub fn min_value() -> Self { 0 }
    //
    //         /// Returns the largest value that can be represented by this integer type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         pub fn max_value() -> Self { !0 }
    //
    //         /// Converts a string slice in a given base to an integer.
    //         ///
    //         /// Leading and trailing whitespace represent an error.
    //         ///
    //         /// # Arguments
    //         ///
    //         /// * src - A string slice
    //         /// * radix - The base to use. Must lie in the range [2 .. 36]
    //         ///
    //         /// # Return value
    //         ///
    //         /// `Err(ParseIntError)` if the string did not represent a valid number.
    //         /// Otherwise, `Ok(n)` where `n` is the integer represented by `src`.
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
    //         pub fn count_ones(self) -> u32 {
    //             unsafe { $ctpop(self as $ActualT) as u32 }
    //         }
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
    //             unsafe { $ctlz(self as $ActualT) as u32 }
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
    //             // As of LLVM 3.6 the codegen for the zero-safe cttz8 intrinsic
    //             // emits two conditional moves on x86_64. By promoting the value to
    //             // u16 and setting bit 8, we get better code without any conditional
    //             // operations.
    //             // FIXME: There's a LLVM patch (http://reviews.llvm.org/D9284)
    //             // pending, remove this workaround once LLVM generates better code
    //             // for cttz8.
    //             unsafe {
    //                 if $BITS == 8 {
    //                     intrinsics::cttz16(self as u16 | 0x100) as u32
    //                 } else {
    //                     $cttz(self as $ActualT) as u32
    //                 }
    //             }
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
    //             // Protect against undefined behaviour for over-long bit shifts
    //             let n = n % $BITS;
    //             (self << n) | (self >> (($BITS - n) % $BITS))
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
    //             // Protect against undefined behaviour for over-long bit shifts
    //             let n = n % $BITS;
    //             (self >> n) | (self << (($BITS - n) % $BITS))
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
    //             unsafe { $bswap(self as $ActualT) as Self }
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
    //                 0 => None,
    //                 v => Some(self / v),
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
    //                 Some(x)                       => x,
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
    //         /// ```rust
    //         /// assert_eq!(2i32.pow(4), 16);
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
    //         /// Returns `true` iff `self == 2^k` for some `k`.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn is_power_of_two(self) -> bool {
    //             (self.wrapping_sub(Self::one())) & self == Self::zero() &&
    //                 !(self == Self::zero())
    //         }
    //
    //         /// Returns the smallest power of two greater than or equal to `self`.
    //         /// Unspecified behavior on overflow.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn next_power_of_two(self) -> Self {
    //             let bits = size_of::<Self>() * 8;
    //             let one: Self = Self::one();
    //             one << ((bits - self.wrapping_sub(one).leading_zeros() as usize) % bits)
    //         }
    //
    //         /// Returns the smallest power of two greater than or equal to `n`. If
    //         /// the next power of two is greater than the type's maximum value,
    //         /// `None` is returned, otherwise the power of two is wrapped in `Some`.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         pub fn checked_next_power_of_two(self) -> Option<Self> {
    //             let npot = self.next_power_of_two();
    //             if npot >= self {
    //                 Some(npot)
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // }

    // impl u64 {
    //     uint_impl! { u64, 64,
    //         intrinsics::ctpop64,
    //         intrinsics::ctlz64,
    //         intrinsics::cttz64,
    //         intrinsics::bswap64,
    //         intrinsics::u64_add_with_overflow,
    //         intrinsics::u64_sub_with_overflow,
    //         intrinsics::u64_mul_with_overflow }
    // }

    macro_rules! wrapping_shr_test {
	($value:expr, $rhs:expr, $result:expr) => ({
	    let x: u64 = $value;
	    let rhs: u32  = $rhs;
	    let result: u64 = x.wrapping_shr(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn wrapping_shr_test1() {
	wrapping_shr_test!( 0x8000000000000000, 0, 0x8000000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 1, 0x4000000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 2, 0x2000000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 3, 0x1000000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 4, 0x0800000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 5, 0x0400000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 6, 0x0200000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 7, 0x0100000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 8, 0x0080000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 9, 0x0040000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 10, 0x0020000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 11, 0x0010000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 12, 0x0008000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 13, 0x0004000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 14, 0x0002000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 15, 0x0001000000000000 );
	wrapping_shr_test!( 0x8000000000000000, 16, 0x0000800000000000 );
	wrapping_shr_test!( 0x8000000000000000, 17, 0x0000400000000000 );
	wrapping_shr_test!( 0x8000000000000000, 18, 0x0000200000000000 );
	wrapping_shr_test!( 0x8000000000000000, 19, 0x0000100000000000 );
	wrapping_shr_test!( 0x8000000000000000, 20, 0x0000080000000000 );
	wrapping_shr_test!( 0x8000000000000000, 21, 0x0000040000000000 );
	wrapping_shr_test!( 0x8000000000000000, 22, 0x0000020000000000 );
	wrapping_shr_test!( 0x8000000000000000, 23, 0x0000010000000000 );
	wrapping_shr_test!( 0x8000000000000000, 24, 0x0000008000000000 );
	wrapping_shr_test!( 0x8000000000000000, 25, 0x0000004000000000 );
	wrapping_shr_test!( 0x8000000000000000, 26, 0x0000002000000000 );
	wrapping_shr_test!( 0x8000000000000000, 27, 0x0000001000000000 );
	wrapping_shr_test!( 0x8000000000000000, 28, 0x0000000800000000 );
	wrapping_shr_test!( 0x8000000000000000, 29, 0x0000000400000000 );
	wrapping_shr_test!( 0x8000000000000000, 30, 0x0000000200000000 );
	wrapping_shr_test!( 0x8000000000000000, 31, 0x0000000100000000 );
	wrapping_shr_test!( 0x8000000000000000, 32, 0x0000000080000000 );
	wrapping_shr_test!( 0x8000000000000000, 33, 0x0000000040000000 );
	wrapping_shr_test!( 0x8000000000000000, 34, 0x0000000020000000 );
	wrapping_shr_test!( 0x8000000000000000, 35, 0x0000000010000000 );
	wrapping_shr_test!( 0x8000000000000000, 36, 0x0000000008000000 );
	wrapping_shr_test!( 0x8000000000000000, 37, 0x0000000004000000 );
	wrapping_shr_test!( 0x8000000000000000, 38, 0x0000000002000000 );
	wrapping_shr_test!( 0x8000000000000000, 39, 0x0000000001000000 );
	wrapping_shr_test!( 0x8000000000000000, 40, 0x0000000000800000 );
	wrapping_shr_test!( 0x8000000000000000, 41, 0x0000000000400000 );
	wrapping_shr_test!( 0x8000000000000000, 42, 0x0000000000200000 );
	wrapping_shr_test!( 0x8000000000000000, 43, 0x0000000000100000 );
	wrapping_shr_test!( 0x8000000000000000, 44, 0x0000000000080000 );
	wrapping_shr_test!( 0x8000000000000000, 45, 0x0000000000040000 );
	wrapping_shr_test!( 0x8000000000000000, 46, 0x0000000000020000 );
	wrapping_shr_test!( 0x8000000000000000, 47, 0x0000000000010000 );
	wrapping_shr_test!( 0x8000000000000000, 48, 0x0000000000008000 );
	wrapping_shr_test!( 0x8000000000000000, 49, 0x0000000000004000 );
	wrapping_shr_test!( 0x8000000000000000, 50, 0x0000000000002000 );
	wrapping_shr_test!( 0x8000000000000000, 51, 0x0000000000001000 );
	wrapping_shr_test!( 0x8000000000000000, 52, 0x0000000000000800 );
	wrapping_shr_test!( 0x8000000000000000, 53, 0x0000000000000400 );
	wrapping_shr_test!( 0x8000000000000000, 54, 0x0000000000000200 );
	wrapping_shr_test!( 0x8000000000000000, 55, 0x0000000000000100 );
	wrapping_shr_test!( 0x8000000000000000, 56, 0x0000000000000080 );
	wrapping_shr_test!( 0x8000000000000000, 57, 0x0000000000000040 );
	wrapping_shr_test!( 0x8000000000000000, 58, 0x0000000000000020 );
	wrapping_shr_test!( 0x8000000000000000, 59, 0x0000000000000010 );
	wrapping_shr_test!( 0x8000000000000000, 60, 0x0000000000000008 );
	wrapping_shr_test!( 0x8000000000000000, 61, 0x0000000000000004 );
	wrapping_shr_test!( 0x8000000000000000, 62, 0x0000000000000002 );
	wrapping_shr_test!( 0x8000000000000000, 63, 0x0000000000000001 );
	wrapping_shr_test!( 0x8000000000000000, 64, 0x8000000000000000 );
    }
}
