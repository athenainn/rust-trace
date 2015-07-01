#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MIN_POSITIVE;

    // pub const MIN_POSITIVE: f64 = 1.17549435e-38_f64;

    #[test]
    fn min_positivet_est1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_00000000001_0000000000000000000000000000000000000000000000000000;
	}

	assert_eq!(MIN_POSITIVE, value);
    }
}
