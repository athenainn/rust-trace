#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MIN;

    // pub const MIN: f64 = -3.40282347e+38_f64;

    #[test]
    fn min_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b1_11111111110_1111111111111111111111111111111111111111111111111111;
	}

	assert_eq!(MIN, value);
    }
}
