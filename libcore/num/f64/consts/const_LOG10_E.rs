#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::LOG10_E;

    // log10(e)
    // pub const LOG10_E: f64 = 0.434294481903251827651128918916605082_f64;

    #[test]
    fn log10_e_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111101_1011110010110111101100010101001001101110010100001110;
	}

	assert_eq!(LOG10_E, value);
    }
}
