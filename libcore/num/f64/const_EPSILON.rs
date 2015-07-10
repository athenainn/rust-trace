#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::EPSILON;

    // pub const EPSILON: f64 = 2.2204460492503131e-16_f64;

    #[test]
    fn epsilon_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111001011_0000000000000000000000000000000000000000000000000000;
	}

	assert_eq!(EPSILON, value);
    }
}
