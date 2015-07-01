#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::SQRT_2;

    // sqrt(2.0)
    // pub const SQRT_2: f64 = 1.41421356237309504880168872420969808_f64;

    #[test]
    fn sqrt_2_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111111_0110101000001001111001100110011111110011101111001101;
	}

	assert_eq!(SQRT_2, value);
    }
}
