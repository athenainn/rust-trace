#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_2_PI;

    // 2.0/pi
    // pub const FRAC_2_PI: f32 = 0.636619772367581343075535053490057448_f32;

    #[test]
    fn frac_2_pi_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111110_01000101111100110000011;
	}

	assert_eq!(FRAC_2_PI, value);
    }
}
