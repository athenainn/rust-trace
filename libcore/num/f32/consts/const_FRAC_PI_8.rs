#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_8;

    // pi/8.0
    // pub const FRAC_PI_8: f32 = 0.39269908169872415480783042290993786_f32;

    #[test]
    fn frac_pi_8_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111101_10010010000111111011011;
	}

	assert_eq!(FRAC_PI_8, value);
    }
}
