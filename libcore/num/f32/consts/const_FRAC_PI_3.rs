#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_3;

    // pi/3.0
    // pub const FRAC_PI_3: f32 = 1.04719755119659774615421446109316763_f32;

    #[test]
    fn frac_pi_3_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111111_00001100000101010010010;
	}

	assert_eq!(FRAC_PI_3, value);
    }
}
