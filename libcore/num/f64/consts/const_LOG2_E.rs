#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::LOG2_E;

    // log2(e)
    // pub const LOG2_E: f64 = 1.44269504088896340735992468100189214_f64;

    #[test]
    fn log2_e_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111111_0111000101010100011101100101001010111000001011111110;
	}

	assert_eq!(LOG2_E, value);
    }
}
