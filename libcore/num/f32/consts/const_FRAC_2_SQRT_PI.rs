#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_2_SQRT_PI;

    // 2.0/sqrt(pi)
    // pub const FRAC_2_SQRT_PI: f32 = 1.12837916709551257389615890312154517_f32;

    #[test]
    fn frac_2_sqrt_pi_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111111_00100000110111010111011;
	}

	assert_eq!(FRAC_2_SQRT_PI, value);
    }
}
