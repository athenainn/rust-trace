#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::f32x4;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

    #[test]
    fn mul_test1() {
	let x: f32x4 = f32x4(
	    1.23, 4.56, 7.89, 0.12
	);
	let y: f32x4 = f32x4(
	    2.0, 2.0, 2.0, 2.0
	);
	let z: f32x4 = x * y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "f32x4(\
	    2.46, 9.12, 15.78, 0.24\
	    )".to_string());
    }
}
