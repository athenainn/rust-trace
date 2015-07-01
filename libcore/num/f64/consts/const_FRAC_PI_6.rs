#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_PI_6;

    // pi/6.0
    // pub const FRAC_PI_6: f64 = 0.52359877559829887307710723054658381_f64;

    #[test]
    fn frac_pi_6_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111110_0000110000010101001000111000001011010111001101100110;
	}

	assert_eq!(FRAC_PI_6, value);
    }
}
