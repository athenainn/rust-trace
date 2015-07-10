#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::E;

    // pub const E: f32 = 2.71828182845904523536028747135266250_f32;

    #[test]
    fn e_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_10000000_01011011111100001010100;
	}

	assert_eq!(E, value);
    }
}
