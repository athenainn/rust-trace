#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::LOG2_E;

    // log2(e)
    // pub const LOG2_E: f32 = 1.44269504088896340735992468100189214_f32;

    #[test]
    fn log2_e_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111111_01110001010101000111011;
	}

	assert_eq!(LOG2_E, value);
    }
}
