#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::NAN;

    // pub const NAN: f32 = 0.0_f32/0.0_f32;

    #[test]
    fn nan_test1() {
	let mut value: u32 = 0;

	unsafe {
	    *(&mut value as *mut u32) =
		0b0_11111111_10000000000000000000000;
	}

	assert_eq!(unsafe { *(&NAN as *const f32 as *const u32) }, value);
    }
}
