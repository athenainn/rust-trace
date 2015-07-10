#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::SQRT_2;

    // sqrt(2.0)
    // pub const SQRT_2: f32 = 1.41421356237309504880168872420969808_f32;

    #[test]
    fn sqrt_2_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_11111110_1101010000010011110011;
	}

	assert_eq!(SQRT_2, value);
    }
}
