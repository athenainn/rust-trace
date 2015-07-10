#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::i64x2;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct i64x2(pub i64, pub i64);

    #[test]
    fn rem_test1() {
	let x: i64x2 = i64x2(
	     0,  1
	);
	let y: i64x2 = i64x2(
	    2, 2
	);
	let z: i64x2 = x % y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "i64x2(0, 1)".to_string());
    }
}
