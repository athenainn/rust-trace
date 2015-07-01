#![feature(core, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Float;

    // impl Float for f32 {
    //     #[inline]
    //     fn nan() -> f32 { NAN }
    //
    //     #[inline]
    //     fn infinity() -> f32 { INFINITY }
    //
    //     #[inline]
    //     fn neg_infinity() -> f32 { NEG_INFINITY }
    //
    //     #[inline]
    //     fn zero() -> f32 { 0.0 }
    //
    //     #[inline]
    //     fn neg_zero() -> f32 { -0.0 }
    //
    //     #[inline]
    //     fn one() -> f32 { 1.0 }
    //
    //     from_str_radix_float_impl! { f32 }
    //
    //     /// Returns `true` if the number is NaN.
    //     #[inline]
    //     fn is_nan(self) -> bool { self != self }
    //
    //     /// Returns `true` if the number is infinite.
    //     #[inline]
    //     fn is_infinite(self) -> bool {
    //         self == Float::infinity() || self == Float::neg_infinity()
    //     }
    //
    //     /// Returns `true` if the number is neither infinite or NaN.
    //     #[inline]
    //     fn is_finite(self) -> bool {
    //         !(self.is_nan() || self.is_infinite())
    //     }
    //
    //     /// Returns `true` if the number is neither zero, infinite, subnormal or NaN.
    //     #[inline]
    //     fn is_normal(self) -> bool {
    //         self.classify() == Fp::Normal
    //     }
    //
    //     /// Returns the floating point category of the number. If only one property
    //     /// is going to be tested, it is generally faster to use the specific
    //     /// predicate instead.
    //     fn classify(self) -> Fp {
    //         const EXP_MASK: u32 = 0x7f800000;
    //         const MAN_MASK: u32 = 0x007fffff;
    //
    //         let bits: u32 = unsafe { mem::transmute(self) };
    //         match (bits & MAN_MASK, bits & EXP_MASK) {
    //             (0, 0)        => Fp::Zero,
    //             (_, 0)        => Fp::Subnormal,
    //             (0, EXP_MASK) => Fp::Infinite,
    //             (_, EXP_MASK) => Fp::Nan,
    //             _             => Fp::Normal,
    //         }
    //     }
    //
    //     /// Returns the mantissa, exponent and sign as integers.
    //     fn integer_decode(self) -> (u64, i16, i8) {
    //         let bits: u32 = unsafe { mem::transmute(self) };
    //         let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
    //         let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
    //         let mantissa = if exponent == 0 {
    //             (bits & 0x7fffff) << 1
    //         } else {
    //             (bits & 0x7fffff) | 0x800000
    //         };
    //         // Exponent bias + mantissa shift
    //         exponent -= 127 + 23;
    //         (mantissa as u64, exponent, sign)
    //     }
    //
    //     /// Rounds towards minus infinity.
    //     #[inline]
    //     fn floor(self) -> f32 {
    //         unsafe { intrinsics::floorf32(self) }
    //     }
    //
    //     /// Rounds towards plus infinity.
    //     #[inline]
    //     fn ceil(self) -> f32 {
    //         unsafe { intrinsics::ceilf32(self) }
    //     }
    //
    //     /// Rounds to nearest integer. Rounds half-way cases away from zero.
    //     #[inline]
    //     fn round(self) -> f32 {
    //         unsafe { intrinsics::roundf32(self) }
    //     }
    //
    //     /// Returns the integer part of the number (rounds towards zero).
    //     #[inline]
    //     fn trunc(self) -> f32 {
    //         unsafe { intrinsics::truncf32(self) }
    //     }
    //
    //     /// The fractional part of the number, satisfying:
    //     ///
    //     /// ```
    //     /// let x = 1.65f32;
    //     /// assert!(x == x.trunc() + x.fract())
    //     /// ```
    //     #[inline]
    //     fn fract(self) -> f32 { self - self.trunc() }
    //
    //     /// Computes the absolute value of `self`. Returns `Float::nan()` if the
    //     /// number is `Float::nan()`.
    //     #[inline]
    //     fn abs(self) -> f32 {
    //         unsafe { intrinsics::fabsf32(self) }
    //     }
    //
    //     /// Returns a number that represents the sign of `self`.
    //     ///
    //     /// - `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    //     /// - `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    //     /// - `Float::nan()` if the number is `Float::nan()`
    //     #[inline]
    //     fn signum(self) -> f32 {
    //         if self.is_nan() {
    //             Float::nan()
    //         } else {
    //             unsafe { intrinsics::copysignf32(1.0, self) }
    //         }
    //     }
    //
    //     /// Returns `true` if `self` is positive, including `+0.0` and
    //     /// `Float::infinity()`.
    //     #[inline]
    //     fn is_positive(self) -> bool {
    //         self > 0.0 || (1.0 / self) == Float::infinity()
    //     }
    //
    //     /// Returns `true` if `self` is negative, including `-0.0` and
    //     /// `Float::neg_infinity()`.
    //     #[inline]
    //     fn is_negative(self) -> bool {
    //         self < 0.0 || (1.0 / self) == Float::neg_infinity()
    //     }
    //
    //     /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    //     /// error. This produces a more accurate result with better performance than
    //     /// a separate multiplication operation followed by an add.
    //     #[inline]
    //     fn mul_add(self, a: f32, b: f32) -> f32 {
    //         unsafe { intrinsics::fmaf32(self, a, b) }
    //     }
    //
    //     /// Returns the reciprocal (multiplicative inverse) of the number.
    //     #[inline]
    //     fn recip(self) -> f32 { 1.0 / self }
    //
    //     #[inline]
    //     fn powi(self, n: i32) -> f32 {
    //         unsafe { intrinsics::powif32(self, n) }
    //     }
    //
    //     #[inline]
    //     fn powf(self, n: f32) -> f32 {
    //         unsafe { intrinsics::powf32(self, n) }
    //     }
    //
    //     #[inline]
    //     fn sqrt(self) -> f32 {
    //         if self < 0.0 {
    //             NAN
    //         } else {
    //             unsafe { intrinsics::sqrtf32(self) }
    //         }
    //     }
    //
    //     #[inline]
    //     fn rsqrt(self) -> f32 { self.sqrt().recip() }
    //
    //     /// Returns the exponential of the number.
    //     #[inline]
    //     fn exp(self) -> f32 {
    //         unsafe { intrinsics::expf32(self) }
    //     }
    //
    //     /// Returns 2 raised to the power of the number.
    //     #[inline]
    //     fn exp2(self) -> f32 {
    //         unsafe { intrinsics::exp2f32(self) }
    //     }
    //
    //     /// Returns the natural logarithm of the number.
    //     #[inline]
    //     fn ln(self) -> f32 {
    //         unsafe { intrinsics::logf32(self) }
    //     }
    //
    //     /// Returns the logarithm of the number with respect to an arbitrary base.
    //     #[inline]
    //     fn log(self, base: f32) -> f32 { self.ln() / base.ln() }
    //
    //     /// Returns the base 2 logarithm of the number.
    //     #[inline]
    //     fn log2(self) -> f32 {
    //         unsafe { intrinsics::log2f32(self) }
    //     }
    //
    //     /// Returns the base 10 logarithm of the number.
    //     #[inline]
    //     fn log10(self) -> f32 {
    //         unsafe { intrinsics::log10f32(self) }
    //     }
    //
    //     /// Converts to degrees, assuming the number is in radians.
    //     #[inline]
    //     fn to_degrees(self) -> f32 { self * (180.0f32 / consts::PI) }
    //
    //     /// Converts to radians, assuming the number is in degrees.
    //     #[inline]
    //     fn to_radians(self) -> f32 {
    //         let value: f32 = consts::PI;
    //         self * (value / 180.0f32)
    //     }
    // }

    #[test]
    fn fract_test1() {
	let value: f32 = 1.01;
	let result: f32 = value.fract();

	assert_eq!(result, 0.00999999);
    }

    #[test]
    fn fract_test2() {
	let value: f32 = 1.49;
	let result: f32 = value.fract();

	assert_eq!(result, 0.49);
    }

    #[test]
    fn fract_test3() {
	let value: f32 = 1.50;
	let result: f32 = value.fract();

	assert_eq!(result, 0.50);
    }

    #[test]
    fn fract_test4() {
	let value: f32 = 1.99;
	let result: f32 = value.fract();

	assert_eq!(result, 0.99);
    }

    #[test]
    fn fract_test5() {
	let value: f32 = -1.01;
	let result: f32 = value.fract();

	assert_eq!(result, -0.00999999);
    }

    #[test]
    fn fract_test6() {
	let value: f32 = -1.49;
	let result: f32 = value.fract();

	assert_eq!(result, -0.49);
    }

    #[test]
    fn fract_test7() {
	let value: f32 = -1.50;
	let result: f32 = value.fract();

	assert_eq!(result, -0.5);
    }

    #[test]
    fn fract_test8() {
	let value: f32 = -1.99;
	let result: f32 = value.fract();

	assert_eq!(result, -0.99);
    }
}
