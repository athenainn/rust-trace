#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::DecodableFloat;

    // pub trait DecodableFloat: Float + Copy {
    //     /// Returns `x * 2^exp`. Almost same to `std::{f32,f64}::ldexp`.
    //     /// This is used for testing.
    //     fn ldexpi(f: i64, exp: isize) -> Self;
    //     /// The minimum positive normalized value.
    //     fn min_pos_norm_value() -> Self;
    // }

    // impl DecodableFloat for f32 {
    //     fn ldexpi(f: i64, exp: isize) -> Self { f as Self * (exp as Self).exp2() }
    //     fn min_pos_norm_value() -> Self { f32::MIN_POSITIVE }
    // }

    #[test]
    fn ldexpi_test1() {
	let f: i64 = 10;
	let exp: isize = 8;
	let result: f32 = DecodableFloat::ldexpi(f, exp);

	assert_eq!(result, 2560.0);
    }
}
