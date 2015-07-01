#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_2_SQRT_PI;

    // 2.0/sqrt(pi)
    // pub const FRAC_2_SQRT_PI: f64 = 1.12837916709551257389615890312154517_f64;

    #[test]
    fn frac_2_sqrt_pi_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111111_0010000011011101011101010000010000101001101101101101;
	}

	assert_eq!(FRAC_2_SQRT_PI, value);
    }
}
