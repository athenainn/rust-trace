#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::E;

    // pub const E: f64 = 2.71828182845904523536028747135266250_f64;

    #[test]
    fn e_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_10000000000_0101101111110000101010001011000101000101011101101001;
	}

	assert_eq!(E, value);
    }
}
