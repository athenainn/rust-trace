#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::i16x8;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct i16x8(pub i16, pub i16, pub i16, pub i16,
    //                  pub i16, pub i16, pub i16, pub i16);

    #[test]
    fn mul_test1() {
	let x: i16x8 = i16x8(
	     0,  1,  2,  3,  4,  5,  6,  7
	);
	let y: i16x8 = i16x8(
	    2, 2, 2, 2, 2, 2, 2, 2
	);
	let z: i16x8 = x * y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "i16x8(\
	    0, 2, 4, 6, 8, 10, 12, 14\
	    )".to_string());
    }
}
