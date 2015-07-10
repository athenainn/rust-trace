#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::NEG_INFINITY;

    // pub const NEG_INFINITY: f64 = -1.0_f64/0.0_f64;

    #[test]
    fn neg_infinity_test1() {
	let mut value: u64 = 0;

	unsafe {
	    *(&mut value as *mut u64) =
		0b1_11111111111_0000000000000000000000000000000000000000000000000000;
	}

	assert_eq!(unsafe { *(&NEG_INFINITY as *const f64 as *const u64) }, value);
    }
}
