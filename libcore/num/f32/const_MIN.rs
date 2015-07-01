#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::MIN;

    // pub const MIN: f32 = -3.40282347e+38_f32;

    #[test]
    fn min_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b1_11111110_11111111111111111111111;
	}

	assert_eq!(MIN, value);
    }
}
