#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_6;

    // pi/6.0
    // pub const FRAC_PI_6: f32 = 0.52359877559829887307710723054658381_f32;

    #[test]
    fn frac_pi_6_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111110_00001100000101010010010;
	}

	assert_eq!(FRAC_PI_6, value);
    }
}
