#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::LN_10;

    // ln(10.0)
    // pub const LN_10: f64 = 2.30258509299404568401799145468436421_f64;

    #[test]
    fn ln_10_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_10000000000_0010011010111011000110111011101101010101010100010110;
	}

	assert_eq!(LN_10, value);
    }
}
