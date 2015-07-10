#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::MIN_POSITIVE;

    // pub const MIN_POSITIVE: f32 = 1.17549435e-38_f32;

    #[test]
    fn min_positivet_est1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_00000001_00000000000000000000000;
	}

	assert_eq!(MIN_POSITIVE, value);
    }
}
