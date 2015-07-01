#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_2;

    // pi/2.0
    // pub const FRAC_PI_2: f32 = 1.57079632679489661923132169163975144_f32;

    #[test]
    fn frac_pi_2_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111111_10010010000111111011011;
	}

	assert_eq!(FRAC_PI_2, value);
    }
}
