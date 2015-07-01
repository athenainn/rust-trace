#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_1_SQRT_2;

    // 1.0/sqrt(2.0)
    // pub const FRAC_1_SQRT_2: f32 = 0.707106781186547524400844362104849039_f32;

    #[test]
    fn frac_1_sqrt_2_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111110_01101010000010011110011;
	}

	assert_eq!(FRAC_1_SQRT_2, value);
    }
}
