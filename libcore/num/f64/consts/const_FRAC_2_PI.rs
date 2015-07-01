#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_2_PI;

    // 2.0/pi
    // pub const FRAC_2_PI: f64 = 0.636619772367581343075535053490057448_f64;

    #[test]
    fn frac_2_pi_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111110_0100010111110011000001101101110010011100100010000011;
	}

	assert_eq!(FRAC_2_PI, value);
    }
}
