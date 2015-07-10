#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    // pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

    #[test]
    fn pi_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_10000000000_1001001000011111101101010100010001000010110100011000;
	}

	assert_eq!(PI, value);
    }
}
