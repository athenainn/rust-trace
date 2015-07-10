#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::estimator::estimate_scaling_factor;

    // pub fn estimate_scaling_factor(mant: u64, exp: i16) -> i16 {
    //     // 2^(nbits-1) < mant <= 2^nbits if mant > 0
    //     let nbits = 64 - (mant - 1).leading_zeros() as i64;
    //     // 1292913986 = floor(2^32 * log_10 2)
    //     // therefore this always underestimates (or is exact), but not much.
    //     (((nbits + exp as i64) * 1292913986) >> 32) as i16
    // }

    #[test]
    fn estimate_scaling_factor_test1() {
	let mant: u64 = 0x0000000000000001;
	let exp: i16 = 1; // 2 ^ 1
	let result: i16 = estimate_scaling_factor(mant, exp);

	assert_eq!(result, 0);
	assert_eq!(result, (mant as f32 * (1 << exp) as f32).log10() as i16);
    }

    #[test]
    fn estimate_scaling_factor_test2() {
	let mant: u64 = 0x0000000000000001;
	let exp: i16 = 5; // 2 ^ 5
	let result: i16 = estimate_scaling_factor(mant, exp);

	assert_eq!(result, 1);
	assert_eq!(result, (mant as f32 * (1 << exp) as f32).log10() as i16);
    }

    #[test]
    fn estimate_scaling_factor_test3() {
	let mant: u64 = 0x0000000000000001;
	let exp: i16 = 7; // 2 ^ 7
	let result: i16 = estimate_scaling_factor(mant, exp);

	assert_eq!(result, 2);
	assert_eq!(result, (mant as f32 * (1 << exp) as f32).log10() as i16);
    }

    #[test]
    fn estimate_scaling_factor_test4() {
	let mant: u64 = 0x0000000000000001;
	let exp: i16 = 10; // 2 ^ 10
	
	let result: i16 = estimate_scaling_factor(mant, exp);

	assert_eq!(result, 3);
	assert_eq!(result, (mant as f32 * (1 << exp) as f32).log10() as i16);
    }

    #[test]
    fn estimate_scaling_factor_test5() {
	let mant: u64 = 0x0000000000000001;
	let exp: i16 = 14; // 2 ^ 14
	
	let result: i16 = estimate_scaling_factor(mant, exp);

	assert_eq!(result, 4);
	assert_eq!(result, (mant as f32 * (1 << exp) as f32).log10() as i16);
    }
}
