#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    // pub const PI: f32 = 3.14159265358979323846264338327950288_f32;

    #[test]
    fn pi_test1() {
	let mut value: f32 = 0.0_f32;

	unsafe {
	    *(&mut value as *mut f32 as *mut u32) =
		0b0_10000000_10010010000111111011011;
	}

	assert_eq!(PI, value);
    }
}
