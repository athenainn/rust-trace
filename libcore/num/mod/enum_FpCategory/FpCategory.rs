#![feature(core, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Float;

    use core::num::FpCategory::{self, Nan, Infinite, Zero, Subnormal, Normal};

    use core::any::Any;

    // #[derive(Copy, Clone, PartialEq, Debug)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum FpCategory {
    //     /// "Not a Number", often obtained by dividing by zero
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Nan,
    //
    //     /// Positive or negative infinity
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Infinite ,
    //
    //     /// Positive or negative zero
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Zero,
    //
    //     /// De-normalized floating point representation (less precise than `Normal`)
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Subnormal,	
    //
    //     /// A regular floating point number
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Normal,
    // }

    // pub trait Float {
    //     /// Returns the NaN value.
    //     fn nan() -> Self;
    //     /// Returns the infinite value.
    //     fn infinity() -> Self;
    //     /// Returns the negative infinite value.
    //     fn neg_infinity() -> Self;
    //     /// Returns -0.0.
    //     fn neg_zero() -> Self;
    //     /// Returns 0.0.
    //     fn zero() -> Self;
    //     /// Returns 1.0.
    //     fn one() -> Self;
    //     /// Parses the string `s` with the radix `r` as a float.
    //     fn from_str_radix(s: &str, r: u32) -> Result<Self, ParseFloatError>;
    //
    //     /// Returns true if this value is NaN and false otherwise.
    //     fn is_nan(self) -> bool;
    //     /// Returns true if this value is positive infinity or negative infinity and
    //     /// false otherwise.
    //     fn is_infinite(self) -> bool;
    //     /// Returns true if this number is neither infinite nor NaN.
    //     fn is_finite(self) -> bool;
    //     /// Returns true if this number is neither zero, infinite, denormal, or NaN.
    //     fn is_normal(self) -> bool;
    //     /// Returns the category that this number falls into.
    //     fn classify(self) -> FpCategory;
    //
    //     /// Returns the mantissa, exponent and sign as integers, respectively.
    //     fn integer_decode(self) -> (u64, i16, i8);
    //
    //     /// Return the largest integer less than or equal to a number.
    //     fn floor(self) -> Self;
    //     /// Return the smallest integer greater than or equal to a number.
    //     fn ceil(self) -> Self;
    //     /// Return the nearest integer to a number. Round half-way cases away from
    //     /// `0.0`.
    //     fn round(self) -> Self;
    //     /// Return the integer part of a number.
    //     fn trunc(self) -> Self;
    //     /// Return the fractional part of a number.
    //     fn fract(self) -> Self;
    //
    //     /// Computes the absolute value of `self`. Returns `Float::nan()` if the
    //     /// number is `Float::nan()`.
    //     fn abs(self) -> Self;
    //     /// Returns a number that represents the sign of `self`.
    //     ///
    //     /// - `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    //     /// - `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    //     /// - `Float::nan()` if the number is `Float::nan()`
    //     fn signum(self) -> Self;
    //     /// Returns `true` if `self` is positive, including `+0.0` and
    //     /// `Float::infinity()`.
    //     fn is_positive(self) -> bool;
    //     /// Returns `true` if `self` is negative, including `-0.0` and
    //     /// `Float::neg_infinity()`.
    //     fn is_negative(self) -> bool;
    //
    //     /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    //     /// error. This produces a more accurate result with better performance than
    //     /// a separate multiplication operation followed by an add.
    //     fn mul_add(self, a: Self, b: Self) -> Self;
    //     /// Take the reciprocal (inverse) of a number, `1/x`.
    //     fn recip(self) -> Self;
    //
    //     /// Raise a number to an integer power.
    //     ///
    //     /// Using this function is generally faster than using `powf`
    //     fn powi(self, n: i32) -> Self;
    //     /// Raise a number to a floating point power.
    //     fn powf(self, n: Self) -> Self;
    //
    //     /// Take the square root of a number.
    //     ///
    //     /// Returns NaN if `self` is a negative number.
    //     fn sqrt(self) -> Self;
    //     /// Take the reciprocal (inverse) square root of a number, `1/sqrt(x)`.
    //     fn rsqrt(self) -> Self;
    //
    //     /// Returns `e^(self)`, (the exponential function).
    //     fn exp(self) -> Self;
    //     /// Returns 2 raised to the power of the number, `2^(self)`.
    //     fn exp2(self) -> Self;
    //     /// Returns the natural logarithm of the number.
    //     fn ln(self) -> Self;
    //     /// Returns the logarithm of the number with respect to an arbitrary base.
    //     fn log(self, base: Self) -> Self;
    //     /// Returns the base 2 logarithm of the number.
    //     fn log2(self) -> Self;
    //     /// Returns the base 10 logarithm of the number.
    //     fn log10(self) -> Self;
    //
    //     /// Convert radians to degrees.
    //     fn to_degrees(self) -> Self;
    //     /// Convert degrees to radians.
    //     fn to_radians(self) -> Self;
    // }

    macro_rules! fpcategory_test {
	($($T:ty)*) => ($({
	    let nan: $T = <$T>::nan();
	    let result: FpCategory = nan.classify();
	    assert_eq!(result, Nan);

	    let inf: $T = <$T>::infinity();
	    let result: FpCategory = inf.classify();
	    assert_eq!(result, Infinite);

	    let neg_inf: $T = <$T>::neg_infinity();
	    let result: FpCategory = neg_inf.classify();
	    assert_eq!(result, Infinite);

	    let zero: $T = <$T>::zero();
	    let result: FpCategory = zero.classify();
	    assert_eq!(result, Zero);

	    let denorm: $T = 0.0 as $T;
	    unsafe {
		if (&denorm as &Any).is::<f32>() {
		    *(&denorm as *const $T as *mut u32) =
			0b0_00000000_11111111111111111111111;
		} else if (&denorm as &Any).is::<f64>() {
		    *(&denorm as *const $T as *mut u64) =
			0b1_00000000000_1111111111111111111111111111111111111111111111111111;
		} else {
		    unreachable!();
		}
	    }
	    let result: FpCategory = denorm.classify();
	    assert_eq!(result, Subnormal);

	    let norm: $T = 68 as $T;
	    let result: FpCategory = norm.classify();
	    assert_eq!(result, Normal);
	})*)
    }

    #[test]
    fn fpcategory_test1() {
	fpcategory_test!( f32 f64 );
    }
}
