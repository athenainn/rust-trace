#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::EPSILON;

    // pub const EPSILON: f32 = 1.19209290e-07_f32;

    #[test]
    fn epsilon_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01101000_00000000000000000000000;
	}

	assert_eq!(EPSILON, value);
    }
}
