#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::INFINITY;

    // pub const INFINITY: f64 = 1.0_f64/0.0_f64;

    #[test]
    fn infinity_test1() {
	let mut value: u64 = 0;

	unsafe {
	    *(&mut value as *mut u64) =
		0b0_11111111111_0000000000000000000000000000000000000000000000000000;
	}

	assert_eq!(unsafe { *(&INFINITY as *const f64 as *const u64) }, value);
    }
}
