#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::u32x4;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

    #[test]
    fn add_test1() {
	let x: u32x4 = u32x4(
	     0,  1,  2,  3
	);
	let y: u32x4 = u32x4(
	    2, 2, 2, 2
	);
	let z: u32x4 = x + y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "u32x4(2, 3, 4, 5)".to_string());
    }
}
