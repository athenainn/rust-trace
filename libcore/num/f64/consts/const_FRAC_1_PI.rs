#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_1_PI;

    // 1.0/pi
    // pub const FRAC_1_PI: f64 = 0.318309886183790671537767526745028724_f64;

    #[test]
    fn frac_1_pi_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111101_0100010111110011000001101101110010011100100010000011;
	}

	assert_eq!(FRAC_1_PI, value);
    }
}
