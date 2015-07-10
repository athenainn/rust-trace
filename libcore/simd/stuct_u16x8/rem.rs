#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::u16x8;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct u16x8(pub u16, pub u16, pub u16, pub u16,
    //                  pub u16, pub u16, pub u16, pub u16);

    #[test]
    fn rem_test1() {
	let x: u16x8 = u16x8(
	     0,  1,  2,  3,  4,  5,  6,  7
	);
	let y: u16x8 = u16x8(
	    2, 2, 2, 2, 2, 2, 2, 2
	);
	let z: u16x8 = x % y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "u16x8(\
	    0, 1, 0, 1, 0, 1, 0, 1\
	    )".to_string());
    }
}
