#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::LN_10;

    // ln(10.0)
    // pub const LN_10: f32 = 2.30258509299404568401799145468436421_f32;

    #[test]
    fn ln_10_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_10000000_00100110101110110001110;
	}

	assert_eq!(LN_10, value);
    }
}
