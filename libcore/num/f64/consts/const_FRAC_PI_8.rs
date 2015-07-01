#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_PI_8;

    // pi/8.0
    // pub const FRAC_PI_8: f64 = 0.39269908169872415480783042290993786_f64;

    #[test]
    fn frac_pi_8_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111101_1001001000011111101101010100010001000010110100011000;
	}

	assert_eq!(FRAC_PI_8, value);
    }
}
