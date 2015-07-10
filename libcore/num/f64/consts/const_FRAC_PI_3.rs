#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::FRAC_PI_3;

    // pi/3.0
    // pub const FRAC_PI_3: f64 = 1.04719755119659774615421446109316763_f64;

    #[test]
    fn frac_pi_3_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111111_0000110000010101001000111000001011010111001101100110;
	}

	assert_eq!(FRAC_PI_3, value);
    }
}
