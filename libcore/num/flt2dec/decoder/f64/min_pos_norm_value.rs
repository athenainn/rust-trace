#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::DecodableFloat;

    use core::f64::MIN_POSITIVE;

    // pub trait DecodableFloat: Float + Copy {
    //     /// Returns `x * 2^exp`. Almost same to `std::{f32,f64}::ldexp`.
    //     /// This is used for testing.
    //     fn ldexpi(f: i64, exp: isize) -> Self;
    //     /// The minimum positive normalized value.
    //     fn min_pos_norm_value() -> Self;
    // }

    // impl DecodableFloat for f64 {
    //     fn ldexpi(f: i64, exp: isize) -> Self { f as Self * (exp as Self).exp2() }
    //     fn min_pos_norm_value() -> Self { f64::MIN_POSITIVE }
    // }

    #[test]
    fn min_pos_norm_value_test1() {
	let result: f64 = DecodableFloat::min_pos_norm_value();

	assert_eq!(result, MIN_POSITIVE);
    }
}
