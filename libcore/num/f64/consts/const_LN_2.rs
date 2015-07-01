#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::consts::LN_2;

    // ln(2.0)
    // pub const LN_2: f64 = 0.693147180559945309417232121458176568_f64;

    #[test]
    fn ln_2_test1() {
	let mut value: f64 = 0.0_f64;

	unsafe {
	    *(&mut value as *mut f64 as *mut u64) =
		0b0_01111111110_0110001011100100001011111110111110100011100111101111;
	}

	assert_eq!(LN_2, value);
    }
}
