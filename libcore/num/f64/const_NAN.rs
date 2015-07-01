#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::NAN;

    // pub const NAN: f64 = 0.0_f64/0.0_f64;

    #[test]
    fn nan_test1() {
	let mut value: u64 = 0;

	unsafe {
	    *(&mut value as *mut u64) =
		0b0_11111111111_1000000000000000000000000000000000000000000000000000;
	}

	assert_eq!(unsafe { *(&NAN as *const f64 as *const u64) }, value);
    }
}
