#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MAX;

    // pub const MAX: f64 = 1.7976931348623157e+308_f64;

    #[test]
    fn max_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_11111111110_1111111111111111111111111111111111111111111111111111;
	}

	assert_eq!(MAX, value);
    }
}
