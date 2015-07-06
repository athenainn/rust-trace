#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::f64x2;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct f64x2(pub f64, pub f64);

    #[test]
    fn div_test1() {
	let x: f64x2 = f64x2(
	    1.23, 4.56
	);
	let y: f64x2 = f64x2(
	    2.0, 2.0
	);
	let z: f64x2 = x / y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "f64x2(0.615, 2.28)".to_string());
    }
}
