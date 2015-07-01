#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_4;

    // pi/4.0
    // pub const FRAC_PI_4: f32 = 0.785398163397448309615660845819875721_f32;

    #[test]
    fn frac_pi_4_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111110_10010010000111111011011;
	}

	assert_eq!(FRAC_PI_4, value);
    }
}
