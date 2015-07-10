#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_1_SQRT_2;

    // 1.0/sqrt(2.0)
    // pub const FRAC_1_SQRT_2: f64 = 0.707106781186547524400844362104849039_f64;

    #[test]
    fn frac_1_sqrt_2_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111110_0110101000001001111001100110011111110011101111001101;
	}

	assert_eq!(FRAC_1_SQRT_2, value);
    }
}
