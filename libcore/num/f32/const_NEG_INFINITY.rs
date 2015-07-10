#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::NEG_INFINITY;

    // pub const NEG_INFINITY: f32 = -1.0_f32/0.0_f32;

    #[test]
    fn neg_infinity_test1() {
	let mut value: u32 = 0;

	unsafe {
	    *(&mut value as *mut u32) =
		0b1_11111111_00000000000000000000000;
	}

	assert_eq!(unsafe { *(&NEG_INFINITY as *const f32 as *const u32) }, value);
    }
}
