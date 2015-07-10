#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::LOG10_E;

    // log10(e)
    // pub const LOG10_E: f32 = 0.434294481903251827651128918916605082_f32;

    #[test]
    fn log10_e_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111101_10111100101101111011001;
	}

	assert_eq!(LOG10_E, value);
    }
}
