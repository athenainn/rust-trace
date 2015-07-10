#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_PI_4;

    // pi/4.0
    // pub const FRAC_PI_4: f64 = 0.785398163397448309615660845819875721_f64;

    #[test]
    fn frac_pi_4_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111110_1001001000011111101101010100010001000010110100011000;
	}

	assert_eq!(FRAC_PI_4, value);
    }
}
