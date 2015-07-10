#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_1_PI;

    // 1.0/pi
    // pub const FRAC_1_PI: f32 = 0.318309886183790671537767526745028724_f32;

    #[test]
    fn frac_1_pi_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111101_01000101111100110000011;
	}

	assert_eq!(FRAC_1_PI, value);
    }
}
