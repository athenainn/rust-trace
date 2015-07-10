#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::LN_2;

    // ln(2.0)
    // pub const LN_2: f32 = 0.693147180559945309417232121458176568_f32;

    #[test]
    fn ln_2_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_01111110_01100010111001000011000;
	}

	assert_eq!(LN_2, value);
    }
}
